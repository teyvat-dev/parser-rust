pub use anyhow::Result;
use clap::{load_yaml, App, AppSettings, ArgMatches};
use utils::{readable::Readable, texthash::TextHash};

extern crate strum;
#[macro_use]
extern crate strum_macros;

mod entities;
mod helpers;
mod parse;
mod structs;
mod utils;

fn parse_command(matches: &ArgMatches) -> Result<()> {
    let entity = matches.value_of("entity");
    let verbose = matches.is_present("verbose");

    let mut parser = parse::Parse::new();

    parser = parser.verbose(verbose);
    let texthash = TextHash::new();
    let texthash = texthash.prepare();
    let readables = Readable::new();
    let readables = readables.prepare();

    if let Some(entity) = entity {
        parser = parser.entity(entity);
    }
    parser.run(texthash, readables)
}

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from(yaml)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::ArgRequiredElseHelp);
    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("parse") {
        parse_command(&matches)?;
    }

    Ok(())
}
