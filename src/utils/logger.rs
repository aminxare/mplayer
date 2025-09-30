use std::fs::File;

pub fn init() {
    // Setup logging to file
    let file = File::create("app.log").unwrap();
    let _ = env_logger::Builder::new()
        .target(env_logger::Target::Pipe(Box::new(file)))
        .filter_level(log::LevelFilter::Info)
        .try_init();
}
