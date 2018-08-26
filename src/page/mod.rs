use maud::{html, Markup};

mod home;
mod savethedate;
mod lodging;
mod childcare;

pub use self::home::HomePage;
pub use self::savethedate::SaveTheDatePage;
pub use self::lodging::LodgingPage;
pub use self::childcare::ChildcarePage;

lazy_static! {
    static ref BOOTSTRAP_INCLUDES: Markup = html! {
        link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css";
        meta name="viewport" content="width=device-width, initial-scale=1";
        script src="https://ajax.googleapis.com/ajax/libs/jquery/3.2.0/jquery.min.js" { "" }
        script src="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/js/bootstrap.min.js" { "" }
    };

    static ref TITLE: Markup = html! {
        title { "The Wedding of Alex and Lillian" }
    };

    static ref FOOTER: Markup = html! {
        div.container align="center" {
            p.small {
                "Copyright © 2018 Alex Walcutt | "
                "Powered by Amazon Web Services | "
                a href="https://github.com/awalcutt/WeddingWebsite" { "Source Code on GitHub" }
            }
        }
    };
}

const PAGES: [PageVariant; 5] = [
    PageVariant::Home,
    PageVariant::SaveTheDate,
    PageVariant::Lodging,
    PageVariant::Childcare,
    PageVariant::Registry,
];

#[derive(PartialEq, Eq)]
pub enum PageVariant {
    Home,
    SaveTheDate,
    Lodging,
    Childcare,
    Registry,
}

impl PageVariant {
    fn path(&self) -> &'static str {
        match self {
            &PageVariant::Home => "/",
            &PageVariant::SaveTheDate => "/savethedate",
            &PageVariant::Lodging => "/lodging",
            &PageVariant::Childcare => "/childcare",
            &PageVariant::Registry => "https://registry.theknot.com/lillian-adamski-thorpe-alex-walcutt-september-2018-wa/25218632",
        }
    }

    fn title(&self) -> &'static str {
        match self {
            &PageVariant::Home => "Home",
            &PageVariant::SaveTheDate => "Save the Date",
            &PageVariant::Lodging => "Lodging",
            &PageVariant::Childcare => "Childcare",
            &PageVariant::Registry => "Registry",
        }
    }

    fn target(&self) -> &'static str {
        match self {
            &PageVariant::Registry => "_blank", // Open page in new tab/window
            _ => "_self", // Open page in current window
        }
    }
}

pub trait WeddingWebsitePage {
    const VARIANT: PageVariant;

    fn content() -> Markup;

    fn navbar() -> Markup {
        html! {
            nav.navbar.navbar-default {
                div.container-fluid {
                    div.navbar-header {
                        a.navbar-brand href="#" { "Alex + Lillian" }
                    }
                    ul.nav.navbar-nav {
                        @for page in PAGES.iter() {
                            @if page == &Self::VARIANT {
                                li.active {
                                    a href=(page.path()) { (page.title()) }
                                }
                            } @else {
                                li {
                                    a href=(page.path()) target=(page.target()) { (page.title()) }
                                }
                            }
                        }
                    }
                    p.navbar-text { "More Content Coming Soon!" }
                }
            }
        }
    }

    fn render() -> Markup {
        html! {
            (BOOTSTRAP_INCLUDES)

            (TITLE)

            (Self::navbar())

            div.container align="center" {
                (Self::content())
            }

            (FOOTER)
        }
    }
}