use std::collections::HashMap;

/// Show docs  
/// returns a hashmap of args
pub fn handle_args(args: &mut std::env::Args) -> Result<HashMap<&'static str, String>, ()> {
    let mut override_map = HashMap::new();
    if let Some(arg1) = args.next() {
        match arg1.as_str() {
            "-v" | "--version" => {
                println!("Version");
                Err(())
            }
            "-h" | "--help" => {
                print_help();
                Err(())
            }
            path => {
                override_map.insert("dir", String::from(path));
                Ok(override_map)
            }
            _ => Err(()),
        }
    } else {
        eprintln!("Oops! tell me where is your music...");
        eprintln!("use --help or -h for help");
        Err(())
    }
}

fn print_help() {
    println!(
        "
            mplayer <musics_dir>
            Mplayer is simple terminal base music player\n

            -v | --version      Show version
            -h | --help         Show helps. (this page)          
        "
    )
}
