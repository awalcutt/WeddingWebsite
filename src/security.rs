use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::Request;
use rocket::request;
use rocket::Outcome;

static X_FORWARDED_PROTO: &'static str = "X-Forwarded-Proto";

pub struct Https;

impl<'a, 'r> FromRequest<'a, 'r> for Https {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let client_protocol_headers: Vec<&str> = request.headers().get(X_FORWARDED_PROTO).collect();

        if client_protocol_headers.len() != 1 {
            return Outcome::Failure((Status::UpgradeRequired, ()))
        }

        let client_protocol = client_protocol_headers[0];

        match client_protocol {
            "https" => Outcome::Success(Https),
            _ => Outcome::Failure((Status::UpgradeRequired, ()))
        }
    }
}