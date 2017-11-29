#![feature(plugin)]
#![feature(proc_macro)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;

use maud::{html, Markup};

#[get("/")]
fn index() -> Markup {
    html! {
        link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css";
        meta name="viewport" content="width=device-width, initial-scale=1";
        script src="https://ajax.googleapis.com/ajax/libs/jquery/3.2.0/jquery.min.js" ""
        script src="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/js/bootstrap.min.js" ""

        title "The Wedding of Alex and Lillian"

        nav.navbar.navbar-default div.container-fluid {
            div.navbar-header a.navbar-brand href="#" "Alex and Lillian Wedding"
            ul.nav.navbar-nav {
                li.active a href="#" "Home"
            }
            p.navbar-text "Additional Pages TBD"
        }

        div.container {
            div.jumbotron {
                h1 "The Wedding of Alex and Lillian"
                p "They are getting married!"
            }
            p "This website is a work in progress..."
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}