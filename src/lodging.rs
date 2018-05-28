use maud::{html, Markup};
use Https;

#[get("/lodging")]
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
                li a href="/savethedate" "Save the Date"
                li.active a href="/lodging" "Lodging"
            }
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