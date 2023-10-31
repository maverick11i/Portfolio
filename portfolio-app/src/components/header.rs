use crate::router::AppRoute;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

/// A Yew functional component representing the header/navigation bar of the application.
#[derive(PartialEq, Properties)]
pub struct Props {
    /// Optional URL to the user's GitHub profile.
    pub github_url: Option<String>,
}

/// The header/navigation bar component.
#[function_component]
pub fn Header(props: &Props) -> Html {
    // Create a repository URL based on the provided GitHub URL.
    let repository_url = match &props.github_url {
        Some(url) => url.clone() + "?tab=repositories",
        None => String::new(),
    };

    html! {
        <>
            <nav class="navbar navbar-expand-lg navbar-light bg-white py-3">
                <div class="container px-5">
                    <Link<AppRoute> classes="navbar-brand" to={ AppRoute::Home }>
                        <span class="fw-bolder text-primary">{ "Portfolio" }</span>
                    </Link<AppRoute>>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav ms-auto mb-2 mb-lg-0 small fw-bolder">
                            <li class="nav-item">
                                <Link<AppRoute> classes="nav-link" to={ AppRoute::Home }>
                                    {"Home"}
                                </Link<AppRoute>>
                            </li>
                            <li class="nav-item">
                                <Link<AppRoute> classes="nav-link" to={ AppRoute::About }>
                                    {"About"}
                                </Link<AppRoute>>
                            </li>
                            <a class="nav-link" href={ repository_url }>
                                {"Repository"}
                            </a>
                        </ul>
                    </div>
                </div>
            </nav>
        </>
    }
}
