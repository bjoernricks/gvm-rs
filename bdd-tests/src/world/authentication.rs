use gvm_rs::commands::authenticate::AuthenticateResponse;

#[derive(Debug, Default)]
pub struct AuthenticationState {
    pub response: Option<AuthenticateResponse>,
}