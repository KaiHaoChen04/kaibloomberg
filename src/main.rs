mod app;
mod app_data;
mod ui;
mod utils;

use app::App;
use ui::run_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    run_ui(&mut app).await
}
