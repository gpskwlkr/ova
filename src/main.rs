use anyhow::Result;

mod app;
mod store;
mod utils;

pub use app::App;
pub use store::Store;

fn main() -> Result<()> {
    let app = App::new();
    app.run()?;
    Ok(())
}
