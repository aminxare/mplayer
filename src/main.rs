use anyhow::Ok;

mod app;
mod audio;
mod ui;

fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();

    app::App::new().run()?;
    Ok(())
}
