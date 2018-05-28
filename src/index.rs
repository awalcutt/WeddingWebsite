use maud::{html, Markup};
use Https;

#[get("/")]
fn render(_https: Https) -> Markup {
    html! {
        link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css";
        meta name="viewport" content="width=device-width, initial-scale=1";
        script src="https://ajax.googleapis.com/ajax/libs/jquery/3.2.0/jquery.min.js" ""
        script src="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/js/bootstrap.min.js" ""

        title "The Wedding of Alex and Lillian"

        nav.navbar.navbar-default div.container-fluid {
            div.navbar-header a.navbar-brand href="#" "Alex + Lillian"
            div.navvar-header a.navvar-brand href="/savethedate" "Save the Date"
            div.navbar-header a.navbar-brand href="/lodging" "Lodging"
            p.navbar-text "More Content Coming Soon!"
        }

        div.container align="center" {
            img src="image/WeddingInvitation.png" class="img-responsive" alt="Lillian Adamski and Alexander Walcutt Wedding September Eighth Twenty Eighteen Seattle Washington Ceremony at Ella Bailey Park 3 PM and Reception at Pacific Tower Panoramic Room 5 PM";
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