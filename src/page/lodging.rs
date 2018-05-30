use maud::{html, Markup};
use page::{PageVariant, WeddingWebsitePage};

pub struct LodgingPage;

impl WeddingWebsitePage for LodgingPage {
    const VARIANT: PageVariant = PageVariant::Lodging;

    fn content() -> Markup {
        html! {
            div.jumbotron {
                p "Our recommendation is to take advantage of AirBnB and other homeshare platforms to find a place to stay near the ceremony or reception."
                p "The ceremony will be held at Ella Bailey Park located in the Magnolia neighborhood northwest of downtown."
                p "The reception will be held at the Pacific Tower Panoramic Room located in the Beacon Hill neighborhood south of downtown."

                div class="embed-responsive embed-responsive-4by3" {
                    iframe class="embed-responsive-item" src="https://www.google.com/maps/d/embed?mid=1mdrl32nTHx5U7RebHPm72B_Y0FJQQsqr&hl=en" ""
                }
            }
        }
    }
}