use maud::{html, Markup};
use Https;

use ::{BOOTSTRAP_INCLUDES, TITLE, FOOTER};

#[get("/")]
fn render(_https: Https) -> Markup {
    html! {
        (BOOTSTRAP_INCLUDES)

        (TITLE)

        nav.navbar.navbar-default div.container-fluid {
            div.navbar-header a.navbar-brand href="#" "Alex + Lillian"
            ul.nav.navbar-nav {
                li.active a href="/" "Home"
                li a href="/savethedate" "Save the Date"
                li a href="/lodging" "Lodging"
            }
            p.navbar-text "More Content Coming Soon!"
        }

        div.container align="center" {
            img src="image/WeddingInvitation.png" class="img-responsive" alt="Lillian Adamski and Alexander Walcutt Wedding September Eighth Twenty Eighteen Seattle Washington Ceremony at Ella Bailey Park 3 PM and Reception at Pacific Tower Panoramic Room 5 PM";
        }

        (FOOTER)
    }
}