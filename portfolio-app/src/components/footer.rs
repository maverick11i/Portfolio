use yew::{function_component, html, Html, Properties};

/// A Yew functional component representing the footer of the application.
#[derive(PartialEq, Properties)]
pub struct Props {
    /// Optional URL to the user's GitHub profile.
    pub github_url: Option<String>,
}

/// The footer component of the application.
#[function_component]
pub fn Footer(props: &Props) -> Html {
    html! {
        <>
            <footer class="bg-white py-4 mt-auto">
                <div class="container px-5">
                    <div class="row align-items-center justify-content-between flex-column flex-sm-row">
                        <div class="col-auto">
                            <div class="small m-0">{"Copyright© Portfolio 2023"}</div>
                        </div>
                        <div class="col-auto">
                            <a class="text-gradient" href={ props.github_url.clone() }>
                                <i class="bi bi-github"></i>
                            </a>
                        </div>
                    </div>
                </div>
            </footer>
        </>
    }
}
