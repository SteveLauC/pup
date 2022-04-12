use clap::{Command, Arg, ArgMatches, crate_authors, crate_version, crate_description};


pub fn cli_init() -> ArgMatches {
    Command::new("pup")
        .author(crate_authors!(" "))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("filename")
            .takes_value(true)
            .required(true)
            .help("Speficy the target markdown file")
         )
        .get_matches()
}
