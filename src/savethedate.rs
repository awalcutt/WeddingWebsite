use maud::{html, Markup};
use Https;

use ::{BOOTSTRAP_INCLUDES, TITLE, FOOTER};

#[get("/savethedate")]
fn render(_https: Https) -> Markup {
    html! {
        (BOOTSTRAP_INCLUDES)

        (TITLE)

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

        (FOOTER)
    }
}