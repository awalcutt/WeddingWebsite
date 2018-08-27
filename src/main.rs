#![feature(plugin)]
#![feature(proc_macro_non_items)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;

#[macro_use]
extern crate lazy_static;

mod page;

use std::path::{Path, PathBuf};

use maud::Markup;
use rocket::{Rocket, response::NamedFile};

use page::{WeddingWebsitePage, HomePage, SaveTheDatePage, LocationsPage, ChildcarePage};

static STATIC_ROOT: &'static str = "/var/www/weddingwebsite/static/";

#[get("/")]
fn render_home_page() -> Markup {
    HomePage::render()
}

#[get("/savethedate")]
fn render_save_the_date_page() -> Markup {
    SaveTheDatePage::render()
}

#[get("/locations")]
fn render_lodging_page() -> Markup {
    LocationsPage::render()
}

#[get("/childcare")]
fn render_childcare_page() -> Markup {
    ChildcarePage::render()
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(STATIC_ROOT).join(file)).ok()
}

#[get("/healthy")]
fn health_check() -> &'static str {
    "healthy"
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![
        render_home_page,
        render_save_the_date_page,
        render_lodging_page,
        render_childcare_page,
        files,
        health_check
    ])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn health_check_always_ok() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/healthy").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
