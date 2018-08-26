use maud::{html, Markup};
use page::{PageVariant, WeddingWebsitePage};

pub struct ChildcarePage;

impl WeddingWebsitePage for ChildcarePage {
    const VARIANT: PageVariant = PageVariant::Childcare;

    fn content() -> Markup {
        html! {
            div.jumbotron {
                p { "We don't want anyone to feel left out of the festivities including parents and children. That is why we are pleased to share that we will have childcare services at the reception!" }
                p { "At the reception venue (Panoramic Room at Pacific Tower) children can hang out right down the hall from the main room along with a trusted childcare provider." }
                p { "There will not be childcare services at the ceremony venue (Ella Bailey Park) but it is a public park with a playground in case children (or adults) get restless." }
                p { "Please let us know on your RSVP how many children will be attending and reach out to Lillian or Alex if you have any questions." }
            }
        }
    }
}