use maud::{html, Markup};
use Https;

#[get("/savethedate")]
fn render(_https: Https) -> Markup {
    html! {
        link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css";
        meta name="viewport" content="width=device-width, initial-scale=1";
        script src="https://ajax.googleapis.com/ajax/libs/jquery/3.2.0/jquery.min.js" ""
        script src="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/js/bootstrap.min.js" ""

        title "The Wedding of Alex and Lillian"

        nav.navbar.navbar-default div.container-fluid {
            div.navbar-header a.navbar-brand href="#" "Alex + Lillian"
            ul.nav.navbar-nav {
                li a href="/" "Home"
                li.active a href="/savethedate" "Save the Date"
                li a href="/lodging" "Lodging"
            }
            p.navbar-text "More Content Coming Soon!"
        }

        div.container align="center" {
            img src="image/SaveTheDate.png" class="img-responsive" alt="Alex Walcutt and Lillian Adamski Save the Date Seattle Washington September Eighth Twenty Eighteen";
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