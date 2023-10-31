use yew::prelude::*;
use yew_router::prelude::*;

use crate::page::{about, home};

/// Represents the available routes for the application.
#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    /// The route for the home page.
    #[at("/")]
    Home,
    /// The route for the about page.
    #[at("/about")]
    About,
}

/// Generates the appropriate HTML content based on the provided route.
///
/// # Arguments
///
/// * `routes` - The route for which to generate the HTML content.
///
/// # Returns
///
/// An HTML representation of the content for the specified route.
pub fn switch(routes: AppRoute) -> Html {
    match routes {
        // MainHomePage Routing
        AppRoute::Home => {
            html! {
                <home::Home />
            }
        }

        AppRoute::About => {
            html! {
                <about::About />
            }
        }
    }
}
