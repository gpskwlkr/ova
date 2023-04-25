use anyhow::Result;

mod store;
mod app;
mod utils;

pub use store::Store;
pub use app::App;

fn main() -> Result<()> {
    let app = App::new();
    app.run()?;
    Ok(())
}
