#[macro_use] extern crate log;
#[macro_use] extern crate clap;
extern crate loggerv;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    loggerv::init_with_verbosity(matches.occurrences_of("verbose")).unwrap();
    info!("Hello world (info)");
    warn!("Hello world (warn)");
    error!("Hello world (error)");
}
