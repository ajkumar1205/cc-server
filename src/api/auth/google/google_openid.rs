use std::process::exit;
use std::sync::{Arc, Mutex};

use actix_web::{web, HttpRequest, HttpResponse, Result};
use lazy_static::lazy_static;
use log::error;
use oauth2::basic::BasicTokenType;
use oauth2::{revocation, EmptyExtraTokenFields, StandardTokenResponse, TokenType};
use openidconnect::core::{
    CoreAuthDisplay, CoreClaimName, CoreClaimType, CoreClient, CoreClientAuthMethod,
    CoreGenderClaim, CoreGrantType, CoreIdTokenClaims, CoreIdTokenVerifier, CoreJsonWebKey,
    CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm, CoreJwsSigningAlgorithm, CoreResponseMode, CoreResponseType,
    CoreRevocableToken, CoreSubjectIdentifierType,
};
use openidconnect::core::{CoreAuthenticationFlow, CoreProviderMetadata, CoreUserInfoClaims};
use openidconnect::reqwest::{async_http_client, http_client};
use openidconnect::{
    AdditionalProviderMetadata, AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, EmptyAdditionalClaims, IdTokenFields, IssuerUrl, Nonce, OAuth2TokenResponse,
    PkceCodeChallenge, PkceCodeVerifier, ProviderMetadata, RedirectUrl, RevocationUrl, Scope,
};
use serde::{Deserialize, Serialize};
use url::Url;

fn handle_error<T: std::error::Error>(fail: &T, msg: &'static str) {
    let mut err_msg = format!("Error : {}", msg);
    let mut cur_fail: Option<&dyn std::error::Error> = Some(fail);
    while let Some(cause) = cur_fail {
        err_msg += &format!("\n caused by: {}", cause);
        cur_fail = cause.source();
    }
    println!("{}", err_msg);
    exit(1)
}

impl AdditionalProviderMetadata for RevocationEndpointProviderMetadata {}
type GoogleProviderMetadata = ProviderMetadata<
    RevocationEndpointProviderMetadata,
    CoreAuthDisplay,
    CoreClientAuthMethod,
    CoreClaimName,
    CoreClaimType,
    CoreGrantType,
    CoreJweContentEncryptionAlgorithm,
    CoreJweKeyManagementAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreResponseMode,
    CoreResponseType,
    CoreSubjectIdentifierType,
>;
type TokenResponse = StandardTokenResponse<
    IdTokenFields<
        EmptyAdditionalClaims,
        EmptyExtraTokenFields,
        CoreGenderClaim,
        CoreJweContentEncryptionAlgorithm,
        CoreJwsSigningAlgorithm,
        CoreJsonWebKeyType,
    >,
    BasicTokenType,
>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RevocationEndpointProviderMetadata {
    revocation_endpoint: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OidcState {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub issuer_url: IssuerUrl,
    pub redirect_url: RedirectUrl,
    pub revocation_endpoint: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub code_verifier: Option<String>,
}

impl OidcState {
    pub fn new(
        &self,
        client_id: ClientId,
        client_secret: ClientSecret,
        issuer_url: IssuerUrl,
    ) -> Self {
        let redirect_url =
            RedirectUrl::new("http://localhost:8080/api/auth/google_auth/callback".to_string())
                .expect("Invalid redirect URL");
        let revocation_endpoint = self.revocation_endpoint.to_owned();

        OidcState {
            client_id,
            client_secret,
            issuer_url,
            redirect_url,
            revocation_endpoint,
            access_token: None,
            refresh_token: None,
            code_verifier: None,
        }
    }

    // Use OpenID Connect Discovery to fetch the provider metadata.
    async fn google_provider_metadata(&self) -> GoogleProviderMetadata {
        let provider_metadata = GoogleProviderMetadata::discover(
            &IssuerUrl::new(self.issuer_url.to_string()).expect("Invalid google issuer URL"),
            http_client,
            // async_http_client,
        )
        // .await
        .unwrap_or_else(|err| {
            handle_error(&err, "Failed to discover OpenID Provider");
            unreachable!();
        });
        provider_metadata
    }

    pub async fn client(&self) -> CoreClient {
        let provider_metadata = self.google_provider_metadata().await;

        let client = CoreClient::from_provider_metadata(
            provider_metadata,
            self.client_id.clone(),
            Some(self.client_secret.clone()),
        )
        .set_redirect_uri(
            RedirectUrl::new(self.redirect_url.to_string()).expect("Invalid redirect URL"),
        )
        .set_revocation_uri(
            RevocationUrl::new(self.revocation_endpoint.clone().unwrap())
                .expect("Invalid revocation endpoint URL"),
        );
        client
    }

    pub async fn authorize_url(&self) -> Result<Url> {
        // let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        // self.code_verifier = Some(pkce_code_verifier);
        // lazy_static! {
        //     static ref PKCE_PAIR: (PkceCodeChallenge, PkceCodeVerifier) = {
        //         // Generate a new PKCE code challenge and verifier
        //         let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        //         // Return the pair to be stored in the lazy_static variable
        //         (pkce_code_challenge, pkce_code_verifier)
        //     };
        // }
        // let pkce_code_challenge: PkceCodeChallenge = PKCE_PAIR;

        let client = self.client().await;

        let authorize_url = client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            // .set_pkce_challenge(pkce_code_challenge)
            .url();

        Ok(authorize_url.0)
    }

    pub async fn exchange_code(&self, query_string: String) -> Result<TokenResponse> {
        // let pkce_code_verifier = *PKCE_PAIR;

        let code = AuthorizationCode::new(
            Url::parse(&("http://localhost:8080/api/auth/callback?".to_string() + &query_string))
                .expect("Failed to parse URL")
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, value)| value.into_owned())
                .expect("No authorization code in query string"),
        );

        let client = self.client().await;

        let token_response = client
            .exchange_code(code)
            // .set_pkce_verifier(pkce_code_verifier)
            .request(http_client)
            .unwrap_or_else(|err| {
                handle_error(&err, "Failed to contact token endpoint");
                unreachable!();
            });

        Ok(token_response)
    }

    pub async fn revocation_endpoint(&self) -> String {
        let provider_metadata = self.google_provider_metadata().await;

        let revocation_endpoint = provider_metadata
            .additional_metadata()
            .revocation_endpoint
            .clone();
        println!(
            "Discovered Google revocation endpoint: {}",
            revocation_endpoint
        );

        revocation_endpoint
    }
}

// Route : api//auth/google_auth/login
pub async fn login(
    data: web::Data<Arc<Mutex<OidcState>>>,
) -> Result<HttpResponse, actix_web::Error> {
    // let data = data.lock();
    let data = data.lock().map_err(|e| {
        error!("Mutex lock error: {:?}", e);
        actix_web::error::ErrorInternalServerError("Internal Server Error")
    })?;

    let authorize_url = data.authorize_url().await?;
    Ok(HttpResponse::Found()
        .append_header(("Location", authorize_url.to_string()))
        .finish())
}

// Route : api//auth/google_auth/callback
pub async fn callback(data: web::Data<Arc<OidcState>>, req: HttpRequest) -> Result<HttpResponse> {
    let token_response = data.exchange_code(req.query_string().to_string()).await?;
    Ok(HttpResponse::Ok().body(format!("Token Response: {:?}", token_response)))
}
