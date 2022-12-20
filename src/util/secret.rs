use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use super::config::CONFIG;

pub struct Secret;

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Secret {
    type Error = AuthError;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let key = request.headers().get_one("Authorization");

        if key.is_none() {
            return Outcome::Failure((Status::Unauthorized, AuthError::Missing));
        }

        let key = key.unwrap();

        if !CONFIG.secrets.contains(&key.to_string()) {
            return Outcome::Failure((Status::Unauthorized, AuthError::Invalid));
        }

        return Outcome::Success(Secret);
    }
}
