use anyhow::Ok;

mod ui;
mod app;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();

    app::App::new().run()?;
    Ok(())
}
