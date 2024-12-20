use crate::domain::auth_domain::Request;
use crate::domain::claims::Claims;
use crate::service::service::{AuthService, Service};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, errors::Result as JwtResult, EncodingKey, Header};

pub const TOKEN_EXPIRE_DURATION: i64 = 3600;

impl AuthService for Service {
    async fn generate_token(&self, req: Request, s: String) -> Result<Option<String>, anyhow::Error> {
        match self.http.check_user(req.clone()).await {
            Ok(user) => {
                if user {
                    tracing::debug!("user exist, generating token");
                    let expiration_time = Utc::now() + Duration::seconds(TOKEN_EXPIRE_DURATION);
                    let expiration = expiration_time.timestamp() as usize;
                    // Creating the claim to be used in the JWT
                    let claim = Claims {
                        sub: req.email.to_string(),
                        exp: expiration,  // A hardcoded expiration time; ideally this should be dynamic
                    };

                    // Creating the encoding key from the secret string
                    let encode_key = EncodingKey::from_secret(s.as_ref());

                    // Encode the JWT token, handle errors gracefully
                    let token: JwtResult<String> = encode(&Header::default(), &claim, &encode_key);

                    match token {
                        Ok(t) => {
                            tracing::debug!("token generated, storing to cache");
                            // Set the token in cache (assuming `self.cache.set` works asynchronously)
                            let _ = self.cache.set(req.email.as_str(), t.as_str()).await;
                            let full_token = format!("Bearer {}",t);
                            Ok(Some(full_token))
                        }
                        Err(e) => {
                            tracing::error!("error generate token !");
                            // Handle JWT encoding error (e.g., invalid claim data)
                            Err(anyhow::anyhow!(e).context("error validate user"))
                        }
                    }
                } else {
                    tracing::info!("user not exist!");
                    // User check failed
                    Ok(None)
                }
            }
            Err(_) => {
                tracing::error!("error processing generate_token for req {}!",req.email);
                Ok(None)
            }
        }
    }
}
