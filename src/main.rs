// Remove the feature flag as it's not needed in Rust 2021
// #![feature(proc_macro_hygiene)]

use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::routes;
use maud::Markup;

mod page;

use std::path::{Path, PathBuf};

use crate::page::{WeddingWebsitePage, HomePage, SaveTheDatePage, DayOfPage, ChildcarePage};

static STATIC_ROOT: &'static str = "/var/www/weddingwebsite/static/";

#[rocket::get("/")]
fn render_home_page() -> RawHtml<Markup> {
    RawHtml(HomePage::render())
}

#[rocket::get("/savethedate")]
fn render_save_the_date_page() -> RawHtml<Markup> {
    RawHtml(SaveTheDatePage::render())
}

#[rocket::get("/dayof")]
fn render_day_of_page() -> RawHtml<Markup> {
    RawHtml(DayOfPage::render())
}

#[rocket::get("/childcare")]
fn render_childcare_page() -> RawHtml<Markup> {
    RawHtml(ChildcarePage::render())
}

#[rocket::get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(STATIC_ROOT).join(file)).await.ok()
}

#[rocket::get("/healthy")]
fn health_check() -> &'static str {
    "healthy"
}

fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![
        render_home_page,
        render_save_the_date_page,
        render_day_of_page,
        render_childcare_page,
        files,
        health_check
    ])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket().launch().await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::asynchronous::Client;
    use rocket::http::Status;

    #[rocket::async_test]
    async fn health_check_always_ok() {
        let client = Client::tracked(rocket()).await.expect("valid rocket instance");
        let response = client.get("/healthy").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
    }
}
