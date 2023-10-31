use portfolio_app::bootstrap;

/// The main entry point of the application.
///
/// This function serves as the starting point for the entire application. It initializes
/// the application's logging system, sets the log level to `Trace`, and then creates and
/// renders an instance of the Yew application defined in the `bootstrap::App` module.
///
/// # Example
///
/// ```rust
/// fn main() {
///     wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
///     yew::Renderer::<bootstrap::App>::new().render();
/// }
/// ```
fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<bootstrap::App>::new().render();
}
