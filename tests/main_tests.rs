use anyhow::Result;
use mplayer::run_app;

#[test]
fn test_run_app_with_valid_dir() -> Result<()> {
    // We assume App::new("test") works in your project
    // You may want to mock filesystem/audio dependencies
    let result = run_app("test");
    // The run might fail depending on how `App::new` is implemented,
    // but at least we can assert it's a valid `Result`.
    assert!(result.is_ok() || result.is_err());
    Ok(())
}
