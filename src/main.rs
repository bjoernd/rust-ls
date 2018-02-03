#[macro_use] extern crate log;
#[macro_use] extern crate clap;
extern crate loggerv;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let params = App::from_yaml(yaml).get_matches();
    loggerv::init_with_verbosity(params.occurrences_of("verbose")).unwrap();
    warn!("verbosity = {}", params.occurrences_of("verbose"));
    info!("Hello world (info)");
    warn!("Hello world (warn)");
    error!("Hello world (error)");
}
