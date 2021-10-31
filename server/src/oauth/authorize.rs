use super::verify_redirect_uri;
use super::AuthorizationRequestQuery;
use crate::State;
use axum::extract::Extension;
use axum::extract::Query;
use axum::response::Html;
use houseflow_types::errors::InternalError;
use houseflow_types::errors::OAuthError;
use houseflow_types::errors::ServerError;

const AUTHORIZE_PAGE: &str = include_str!("authorize.html");

#[tracing::instrument(name = "Authorization", skip(state), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    Query(request): Query<AuthorizationRequestQuery>,
) -> Result<Html<&'static str>, ServerError> {
    let google_config = state
        .config
        .google
        .as_ref()
        .ok_or_else(|| InternalError::Other("Google Home API not configured".to_string()))?;
    if *request.client_id != *google_config.client_id {
        return Err(OAuthError::InvalidClient(Some(String::from("invalid client id"))).into());
    }
    verify_redirect_uri(&request.redirect_uri, &google_config.project_id)
        .map_err(|err| OAuthError::InvalidRequest(Some(err.to_string())))?;

    Ok(Html(AUTHORIZE_PAGE))
}
