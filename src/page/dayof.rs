use maud::{html, Markup};
use page::{PageVariant, WeddingWebsitePage};

pub struct DayOfPage;

impl WeddingWebsitePage for DayOfPage {
    const VARIANT: PageVariant = PageVariant::DayOf;

    fn content() -> Markup {
        html! {
            div.jumbotron {
                h3 { "Ceremony (Ella Bailey Park, Magnolia Neighborhood)" }
                p { "The ceremony will be held at Ella Bailey Park located in the Magnolia neighborhood northwest of downtown." }
                p { "Please do not bring anything to throw during the ceremony (flowers, rice, etc.). It is prohibited by the Parks Department and we want to leave the park in a good state for everyone else to enjoy." }
                h4 { "Parking" }
                p { "The park is located in a residential area so there is plenty of free parking along the surrounding streets. Please be mindful of the neighbors." }

                h3 { "Reception (Pacific Tower Panoramic Room, Beacon Hill Neighborhood)" }
                p { "The reception will be held at the Pacific Tower Panoramic Room located in the Beacon Hill neighborhood south of downtown." }
                p { "There will be a variety of food and drink to enjoy. Beer, wine, and non-alcoholic beverages will be free to our guests! Basic mixed drinks will be available for a charge (no cash)." }
                h4 { "Parking" }
                p { "There is free parking on the street. For those that are interested, we also have access to a parking garage. Parking in the garage is $6 for the entire evening and you must use a card (no cash)." }

                div class="embed-responsive embed-responsive-4by3" {
                    iframe class="embed-responsive-item" src="https://www.google.com/maps/d/embed?mid=1mdrl32nTHx5U7RebHPm72B_Y0FJQQsqr&hl=en" { "" }
                }
            }
        }
    }
}
