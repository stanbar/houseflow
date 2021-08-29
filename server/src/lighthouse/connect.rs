use super::{Session, SessionInternals};
use crate::State;
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{ws::WebSocketUpgrade, Extension, TypedHeader},
    response::IntoResponse,
};
use houseflow_types::{
    errors::{AuthError, LighthouseError, ServerError},
    DeviceID,
};
use std::str::FromStr;

pub struct DeviceCredentials(DeviceID, String);

#[async_trait]
impl axum::extract::FromRequest<Body> for DeviceCredentials {
    type Rejection = ServerError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(headers::Authorization(authorization)) =
            TypedHeader::<headers::Authorization<headers::authorization::Basic>>::from_request(req)
                .await
                .map_err(|err| AuthError::InvalidAuthorizationHeader(err.to_string()))?;
        let device_id = DeviceID::from_str(authorization.username()).map_err(|err| {
            AuthError::InvalidAuthorizationHeader(format!("invalid device id: {}", err))
        })?;

        Ok(Self(device_id, authorization.password().to_owned()))
    }
}

#[tracing::instrument(
    name = "WebSocket",
    skip(websocket, state, socket_address, device_password),
    err
)]
pub async fn handle(
    websocket: WebSocketUpgrade,
    Extension(state): Extension<State>,
    Extension(socket_address): Extension<std::net::SocketAddr>,
    DeviceCredentials(device_id, device_password): DeviceCredentials,
) -> Result<impl IntoResponse, ServerError> {
    let device = state
        .config
        .get_device(&device_id)
        .ok_or(AuthError::DeviceNotFound)?;
    crate::verify_password(device.password_hash.as_ref().unwrap(), &device_password)?;
    if state.sessions.contains_key(&device.id) {
        return Err(LighthouseError::AlreadyConnected.into());
    }

    use tracing::Instrument;
    let span = tracing::Span::current();

    Ok(websocket.on_upgrade(move |stream| {
        async move {
            let session_internals = SessionInternals::new();
            let session = Session::new(&session_internals);
            tracing::info!(address = %socket_address, "Device connected");
            state.sessions.insert(device.id.clone(), session.clone());
            match session.run(stream, session_internals).await {
                Ok(_) => tracing::info!("Connection closed"),
                Err(err) => tracing::error!("Connection closed with error: {}", err),
            }
            state.sessions.remove(&device.id);
        }
        .instrument(span)
    }))
}
