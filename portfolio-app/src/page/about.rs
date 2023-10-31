use dotenv_codegen::dotenv;
use portfolio_core::model::{status::FetchState, User};
use portfolio_services::usecase::user::get_user_profile;
use yew::{html, Component, Context, Html};

use crate::components::{footer::Footer, header::Header, profile::Profile};

const USER_INFO: &str = dotenv!("USER_INFO");

/// Represents the About page component.
pub struct About {
    response: FetchState<String>,
}

/// Represents the possible messages that can be sent to the `About` component.
pub enum Msg {
    /// Indicates the start of data fetching.
    FetchStart,
    /// Sets the fetch state based on the result of data fetching.
    SetFetchState(FetchState<String>),
}

impl Component for About {
    type Message = Msg;
    type Properties = ();

    /// Creates a new instance of the `About` component.
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchStart);

        Self {
            response: FetchState::NotFetching,
        }
    }

    /// Handles messages sent to the `About` component.
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchStart => {
                ctx.link().send_future(async {
                    match get_user_profile(USER_INFO).await {
                        Ok(response) => Msg::SetFetchState(FetchState::Success(response)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));

                true
            }
            Msg::SetFetchState(state) => {
                self.response = state;
                true
            }
        }
    }

    /// Renders the HTML view for the `About` component based on the current fetch state.
    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.response {
            FetchState::Failed(_) => {
                html! {
                    <>
                        <p>{ "Fetching Failed" }</p>
                    </>
                }
            }
            FetchState::Fetching => {
                html! {
                    <>
                        <div class="d-flex flex-column h-100">
                            <main class="flex-shrink-0">
                                <Header github_url={ "" } />
                            </main>
                        </div>
                    </>
                }
            }
            FetchState::NotFetching => {
                html! {
                    <>
                        <p>{ "Not Fetching" }</p>
                    </>
                }
            }
            FetchState::Success(response) => match serde_json::from_str::<User>(&response) {
                Ok(json) => {
                    html! {
                        <>
                            <div class="d-flex flex-column h-100">
                                <main class="flex-shrink-0">
                                    <Header github_url={ json.html_url.clone() } />
                                    <Profile />
                                    <Footer github_url={ json.html_url.clone() } />
                                </main>
                            </div>
                        </>
                    }
                }
                Err(e) => {
                    html! {
                        <>
                            <p>{ "Fetch Error" }</p>
                            <p>{ e.to_string() }</p>
                        </>
                    }
                }
            },
        }
    }
}
