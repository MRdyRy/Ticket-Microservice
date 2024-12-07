use crate::domain::authdomain::Request;
use crate::domain::claims::Claims;
use crate::service::service::{AuthService, Service};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::io::Error;

impl AuthService for Service {
    async fn generate_token(&self, req: Request, s: String) -> Result<Option<String>, Error> {
        match self.http.check_user(req).await.or_else(false) {
            Ok(user) => {
                let claim = Claims {
                    sub: req.email.to_string(),
                    exp: 10000000000,
                };
                let encode_key = EncodingKey::from_secret(s.as_ref());
                let token = encode(&Header::default(), &claim, &encode_key).unwrap();

                let _ = self.cache.set(req.email.as_str(), token.as_str()).await;

                Ok(Some(token))
            }
            Err(e) => Ok(None),
        }
    }
}
