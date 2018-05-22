#![feature(plugin)]
#![feature(proc_macro)]
#![feature(proc_macro_non_items)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;

use std::path::{Path, PathBuf};

use maud::{html, Markup};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::response::{NamedFile, Redirect};
use rocket::Request;
use rocket::request;
use rocket::Outcome;
use rocket::Rocket;

static HOSTNAME: &'static str = "https://alexandlillian.wedding";
static STATIC_ROOT: &'static str = "/var/www/weddingwebsite/static/";
static X_FORWARDED_PROTO: &'static str = "X-Forwarded-Proto";

struct Https;

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

#[error(426)]
fn http_redirect(request: &Request) -> Redirect {
    let redirect_uri = HOSTNAME.to_owned() + request.uri().path();
    Redirect::permanent(&redirect_uri)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(STATIC_ROOT).join(file)).ok()
}

#[get("/healthy")]
fn health_check() -> &'static str {
    "healthy"
}

#[get("/")]
fn index(_https: Https) -> Markup {
    html! {
        link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css";
        meta name="viewport" content="width=device-width, initial-scale=1";
        script src="https://ajax.googleapis.com/ajax/libs/jquery/3.2.0/jquery.min.js" ""
        script src="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/js/bootstrap.min.js" ""

        title "The Wedding of Alex and Lillian"

        nav.navbar.navbar-default div.container-fluid {
            div.navbar-header a.navbar-brand href="#" "Alex + Lillian"
            div.navbar-header a.navbar-brand href="/lodging" "Lodging"
            p.navbar-text "More Content Coming Soon!"
        }

        div.container align="center" {
            img src="image/WeddingComic.png" class="img-responsive" alt="Alex Walcutt and Lillian Adamski Save the Date Seattle Washington September Eighth Twenty Eighteen";
        }

        div.container align="center" {
            p.small {
                "Copyright © 2018 Alex Walcutt | "
                "Powered by Amazon Web Services | "
                a href="https://github.com/awalcutt/WeddingWebsite" "Source Code on GitHub"
            }
        }
    }
}

#[get("/lodging")]
fn lodging(_https: Https) -> Markup {
    html! {
        link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css";
        meta name="viewport" content="width=device-width, initial-scale=1";
        script src="https://ajax.googleapis.com/ajax/libs/jquery/3.2.0/jquery.min.js" ""
        script src="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/js/bootstrap.min.js" ""

        title "The Wedding of Alex and Lillian"

        nav.navbar.navbar-default div.container-fluid {
            div.navbar-header a.navbar-brand href="/" "Alex + Lillian"
            div.navbar-header a.navbar-brand href="#" "Lodging"
            p.navbar-text "More Content Coming Soon!"
        }

        div.container align="center" {
            div.jumbotron {
                p "Our recommendation is to take advantage of AirBnB and other homeshare platforms to find a place to stay near the ceremony or reception."
                p "The ceremony will be held at Ella Bailey Park located in the Magnolia neighborhood northwest of downtown."
                p "The reception will be held at the Pacific Tower Panoramic Room located in the Beacon Hill neighborhood south of downtown."

                div class="embed-responsive embed-responsive-4by3" {
                    iframe class="embed-responsive-item" src="https://www.google.com/maps/d/embed?mid=1mdrl32nTHx5U7RebHPm72B_Y0FJQQsqr&hl=en" ""
                }
            }
        }

        div.container align="center" {
            p.small {
                "Copyright © 2018 Alex Walcutt | "
                "Powered by Amazon Web Services | "
                a href="https://github.com/awalcutt/WeddingWebsite" "Source Code on GitHub"
            }
        }
    }
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index, lodging, files, health_check]).catch(errors![http_redirect])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Header;
    use rocket::http::Status;

    use super::X_FORWARDED_PROTO;

    #[test]
    fn index_https_ok() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/").header(Header::new(X_FORWARDED_PROTO, "https")).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn index_http_redirect() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/").header(Header::new(X_FORWARDED_PROTO, "http")).dispatch();
        assert_eq!(response.status(), Status::PermanentRedirect);
    }

    #[test]
    fn index_headerless_redirect() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::PermanentRedirect);
    }

    #[test]
    fn health_check_always_ok() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/healthy").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
