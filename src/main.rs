#![feature(plugin)]
#![feature(proc_macro)]
#![feature(proc_macro_non_items)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;

#[macro_use]
extern crate lazy_static;

mod page;
mod security;

use std::path::{Path, PathBuf};

use maud::Markup;
use rocket::{Request, Rocket, response::NamedFile, response::Redirect};

use security::Https;
use page::{WeddingWebsitePage, HomePage, SaveTheDatePage, LodgingPage};

static HOSTNAME: &'static str = "https://alexandlillian.wedding";
static STATIC_ROOT: &'static str = "/var/www/weddingwebsite/static/";

#[get("/")]
fn render_home_page(_https: Https) -> Markup {
    HomePage::render()
}

#[get("/savethedate")]
fn render_save_the_date_page(_https: Https) -> Markup {
    SaveTheDatePage::render()
}

#[get("/lodging")]
fn render_lodging_page(_https: Https) -> Markup {
    LodgingPage::render()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(STATIC_ROOT).join(file)).ok()
}

#[get("/healthy")]
fn health_check() -> &'static str {
    "healthy"
}

#[error(426)]
fn http_redirect(request: &Request) -> Redirect {
    let redirect_uri = HOSTNAME.to_owned() + request.uri().path();
    Redirect::permanent(&redirect_uri)
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![
        render_home_page,
        render_save_the_date_page,
        render_lodging_page,
        files,
        health_check
    ]).catch(errors![http_redirect])
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

    static X_FORWARDED_PROTO: &'static str = "X-Forwarded-Proto";

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
