use crate::router::AppRoute;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

/// Represents the properties for the `Top` component.
#[derive(PartialEq, Properties)]
pub struct Props {
    /// The name of the user.
    pub name: Option<String>,
    /// The location of the user.
    pub location: Option<String>,
    /// The GitHub URL of the user.
    pub github_url: Option<String>,
}

/// A functional component representing the top section of the portfolio site.
#[function_component]
pub fn Top(props: &Props) -> Html {
    /// Generates the URL for the user's GitHub repositories.
    fn generate_repository_url(github_url: &Option<String>) -> String {
        match github_url {
            Some(url) => format!("{}?tab=repositories", url),
            None => String::new(),
        }
    }

    html! {
        <>
            <header class="py-5 fadein">
                <div class="container px-5 pb-5">
                    <div class="row gx-5 align-items-center">
                        <div class="col-xxl-5">
                            <div class="text-center text-xxl-start">
                                <div class="badge bg-gradient-primary-to-secondary text-white mb-4"><div class="text-uppercase">{"Engineer"}</div></div>
                                <div class="fs-3 fw-light text-muted">{ "GitHub : " }<strong>{ props.name.clone() }</strong></div>
                                <div class="fs-3 fw-light text-muted"><small>{ "Location : " }{ props.location.clone() }</small></div>
                                <h1 class="display-3 fw-bolder mb-5"><span class="text-gradient d-inline">{"This is my portfolio site"}</span></h1>
                                <div class="d-grid gap-3 d-sm-flex justify-content-sm-center justify-content-xxl-start mb-3">
                                    <Link<AppRoute> classes="btn btn-primary btn-lg px-5 py-3 me-sm-3 fs-6 fw-bolder" to={ AppRoute::About }>{"About"}</Link<AppRoute>>
                                    <a class="btn btn-outline-dark btn-lg px-5 py-3 fs-6 fw-bolder" href={ generate_repository_url(&props.github_url) }>{"Repository"}</a>
                                </div>
                            </div>
                        </div>
                        <div class="col-xxl-7">
                            <div class="d-flex justify-content-center mt-xxl-0">
                                <div class="profile">
                                    <img class="profile-img" src="https://cdn-icons-png.flaticon.com/512/7450/7450756.png" alt="..." />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </header>
            <section class="bg-light py-5">
                <div class="container px-5">
                    <div class="row gx-5 justify-content-center">
                        <div class="col-xxl-8">
                            <div class="text-center my-5">
                                <h2 class="display-5 fw-bolder"><span class="text-gradient d-inline">{"This Project"}</span></h2>
                                <p class="text-muted">{"This portfolio is created using the Rust framework Yew."}</p>
                                <div class="d-flex justify-content-center fs-2 gap-4">
                                    <a class="text-gradient" href="https://github.com/maverick11i/Portfolio"><i class="bi bi-github"></i></a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}
