use bytes::BytesMut;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    sync::{mpsc, watch, RwLock},
};

/// 5 seconds timeout for getting response for a request
const REQUEST_TIMEOUT_MILLIS: u64 = 5000;

#[derive(Debug)]
pub enum Error {
    /// Client not found when searching in ConnectionsStore
    ClientNotFound,

    /// Response not received in expected time
    RequestTimeout,
}

/// ClientID which will be used to identify connections
pub type ClientID = String;

/// Response sent from the Client to Server
#[derive(Debug, Clone)]
pub struct Response {
    data: Vec<u8>,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.data).expect("response.data is invalid string")
        )
    }
}

/// Request sent from Server to Client
#[derive(Debug)]
pub struct Request {
    data: Vec<u8>,
}

impl Request {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

/// Thread safe channel which allows sending Requests
pub type RequestSender = mpsc::Sender<Request>;

/// Not thread safe channel which allows reading Requests from RequestSender, this will be used
/// only internally by connection_write_loop()
type RequestReceiver = mpsc::Receiver<Request>;

/// Thread safe channel which allows retrieving Responses
pub type ResponseReceiver = watch::Receiver<Option<Response>>;

/// Not thread safe channel which allows sending Responses to ResponseReceiver, this will be used
/// only internally by connection_read_loop()
type ResponseSender = watch::Sender<Option<Response>>;

/// RequestResponseChannel combines:
/// - RequestSender: used to push new requests to stream
/// - ResponseReceiver: used to retrieve responsese from stream
type RequestResponseChannel = (ResponseReceiver, RequestSender);

/// Store holds thread safe RequestResponseChannels with corresponding ClientIDs
#[derive(Clone)]
pub struct Store {
    inner: Arc<RwLock<HashMap<ClientID, RequestResponseChannel>>>,
}

impl Store {
    /// Creates new store and returns it
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Used to add new Connection to store
    pub async fn add(&self, client_id: ClientID, channel: RequestResponseChannel) {
        self.inner.write().await.insert(client_id, channel);
    }

    /// Sends request over RequestSender channel to connection with specific ClientID
    pub async fn send_request(
        &self,
        client_id: &ClientID,
        request: Request,
    ) -> Result<Response, Error> {
        let (mut rx, tx) = self
            .inner
            .read()
            .await
            .get(client_id)
            .ok_or(Error::ClientNotFound)?
            .clone();

        let timeout = Duration::from_millis(REQUEST_TIMEOUT_MILLIS);

        tx.send(request).await.expect("receiver channel is closed");
        tokio::time::timeout(timeout, rx.changed())
            .await
            .map_err(|_| Error::RequestTimeout)?
            .expect("Sender half has been dropped");
        let response = rx.borrow().clone().expect("Received None response");

        Ok(response)
    }
}

/// Starts connection on stream
pub async fn run(
    stream: (impl AsyncRead + Unpin, impl AsyncWrite + Unpin),
    address: SocketAddr,
    store: Store,
) {
    let client_id = address.port().to_string(); // TODO: replace it with real ClientID
    log::info!(
        "started with client ID: `{}` from `{}`",
        client_id,
        address.to_string()
    );
    let (request_sender, request_receiver) = mpsc::channel::<Request>(32);
    let (response_sender, response_receiver) = watch::channel::<Option<Response>>(None);

    store
        .add(client_id, (response_receiver.clone(), request_sender))
        .await;
    let (stream_receiver, stream_sender) = stream;
    tokio::select! {
        _ = connection_read_loop(stream_receiver, response_sender) => {},
        _ = connection_write_loop(stream_sender, request_receiver) => {},
    };
}

async fn connection_read_loop(
    mut stream: impl AsyncRead + Unpin,
    events: ResponseSender,
) -> Result<(), Error> {
    let mut buf = BytesMut::with_capacity(4096);
    loop {
        let n = stream
            .read_buf(&mut buf)
            .await
            .expect("fail reading buffer");
        let resp = Response {
            data: buf[0..n].to_vec(),
        };
        events.send(Some(resp)).expect("failed sending response");
    }
}

async fn connection_write_loop(
    mut stream: impl AsyncWrite + Unpin,
    mut events: RequestReceiver,
) -> Result<(), Error> {
    loop {
        let request = events.recv().await.unwrap(); // TODO: remove this unwrap
        stream
            .write(&request.data)
            .await
            .expect("fail writing request data to stream");
    }
}
