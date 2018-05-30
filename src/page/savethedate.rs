use maud::{html, Markup};
use page::{PageVariant, WeddingWebsitePage};

pub struct SaveTheDatePage;

impl WeddingWebsitePage for SaveTheDatePage {
    const VARIANT: PageVariant = PageVariant::SaveTheDate;

    fn content() -> Markup {
        html! {
            img src="image/SaveTheDate.png" class="img-responsive" alt="Alex Walcutt and Lillian Adamski Save the Date Seattle Washington September Eighth Twenty Eighteen";
        }
    }
}