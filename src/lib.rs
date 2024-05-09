pub use oauth2::{ClientId, ClientSecret};
use url::form_urlencoded;
mod login_response;
pub use login_response::*;
use std::sync::Arc;
pub mod errors;
use errors::AzureError;

pub async fn authorize_non_interactive(
    client: Arc<reqwest::Client>,
    //  grant_type: &str, fixed on "client_credentials",
    client_id: &oauth2::ClientId,
    client_secret: &oauth2::ClientSecret,
    resource: &str,
    tenant_id: &str,
) -> Result<LoginResponse, AzureError> {
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("grant_type", "client_credentials")
        .append_pair("client_id", client_id.as_str())
        .append_pair("client_secret", client_secret.secret())
        .append_pair("resource", resource)
        .finish();

    let url = url::Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/token",
        tenant_id
    ))
    .map_err(|error| AzureError::GenericErrorWithText(error.to_string()))?;

    client
        .post(url)
        .header("ContentType", "Application / WwwFormUrlEncoded")
        .body(encoded)
        .send()
        .await
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))?
        .text()
        .await
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))
        .and_then(|s| {
            serde_json::from_str::<LoginResponse>(&s).map_err(|e| {
                serde_json::from_str::<errors::ErrorResponse>(&s)
                    .map(|er| AzureError::GenericErrorWithText(er.to_string()))
                    .unwrap_or_else(|_| {
                        AzureError::GenericErrorWithText(format!(
                            "Failed to parse Azure response: {}",
                            e
                        ))
                    })
            })
        })
}
