use anyhow::Result;
use ova::App;

fn main() -> Result<()> {
    let app = App::new();
    app.run()?;
    Ok(())
}
