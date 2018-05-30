use maud::{html, Markup};
use page::{PageVariant, WeddingWebsitePage};

pub struct HomePage;

impl WeddingWebsitePage for HomePage {
    const VARIANT: PageVariant = PageVariant::Home;

    fn content() -> Markup {
        html! {
            img src="image/WeddingInvitation.png" class="img-responsive" alt="Lillian Adamski and Alexander Walcutt Wedding September Eighth Twenty Eighteen Seattle Washington Ceremony at Ella Bailey Park 3 PM and Reception at Pacific Tower Panoramic Room 5 PM";
        }
    }
}