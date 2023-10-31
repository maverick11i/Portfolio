use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, AppRoute};

/// The root component of the application responsible for setting up the Yew router.
///
/// This component initializes the application's router using Yew Router's `BrowserRouter`
/// and defines the routing behavior using the `Switch` component, rendering the appropriate
/// component based on the active route.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// # use yew_router::prelude::*;
/// # use crate::router::{switch, AppRoute};
/// # #[function_component]
/// # pub fn app() -> Html {
/// html! {
///     <BrowserRouter>
///         <Switch<AppRoute> render={switch} />
///     </BrowserRouter>
/// }
/// # }
/// ```
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}
