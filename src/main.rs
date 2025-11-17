// This is the entry point of the application. It initializes the asynchronous runtime, sets up the API calls, and starts the user interface.

use tokio;

mod api;
mod models;
mod services;
mod ui;
mod ws;
mod errors;

#[tokio::main]
async fn main() {
    // Initialize the API and fetch options data
    let options_data = api::fetch_options_data().await.unwrap();

    // Process the fetched options data
    let processed_data = services::process_options_data(options_data);

    // Start the user interface (terminal or web)
    ui::terminal::display_terminal(processed_data).await;
    // Alternatively, you could start the web server
    // ui::web::start_web_server(processed_data).await;
}