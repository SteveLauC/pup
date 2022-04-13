use clap::{crate_authors, crate_description, crate_version, Arg, Command, ArgMatches};
use std::path::PathBuf;
use std::process::exit;

/// initialize the command line application
pub fn cli_init() -> ArgMatches {
    Command::new("pup")
        .author(crate_authors!(" "))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("filename")
                .takes_value(true)
                .required(true)
                .help("Speficy the target markdown file"),
        )
        .get_matches()
}

/// get the target markdown file
/// and make sure it is valid
pub fn get_target_file(app: ArgMatches) -> PathBuf {
    let filename: &str = app.value_of("filename").unwrap();

    let file_path = PathBuf::from(filename);
    if file_path.exists() {
        file_path
    } else {
        eprintln!("filename does not exist");
        exit(1);
    }
}
