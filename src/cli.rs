/*
 * cli.rs: handles everything relevant to command line interface
*/

use clap::{crate_authors, crate_description, crate_version, Arg, ArgMatches, Command};
use std::path::PathBuf;
use std::process::exit;

/*
 * type to represent the command line interface configuration
*/
pub struct CliCfg {
    pub filename: PathBuf,
    pub verbose: bool,
}

impl CliCfg {
    fn new(filename: PathBuf, verbose: bool) -> Self {
        CliCfg { filename, verbose }
    }
}

/*
 * purpose: initialize the command line application
*/
pub fn cli_init() -> ArgMatches {
    Command::new("pup")
        .author(crate_authors!(" "))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("filename")
                .takes_value(true)
                .required(true)
                .help("To speficy the target markdown file"),
        )
        .arg(
            Arg::new("verbose")
            .short('v')
            .long("verbose")
            .takes_value(false)
            .help("Be verbose when outputing the uploading info, showing them as they are detected and uploaded")
        )
        .get_matches()
}

/*
 * purpose: initialize a cli config struct
 * note: we need to make sure `filenamb` exists 
*/
pub fn get_cli_config(app: ArgMatches) -> CliCfg {
    // we can guarantee `filename` is given, so just unwrap here.
    let file: PathBuf = PathBuf::from(app.value_of("filename").unwrap());
    
    if file.exists() {
        CliCfg::new(
            file,
            app.is_present("verbose"),
        )
    } else {
        eprintln!("pup: {:?} does not exist", file); 
        exit(1);
    }
}
