#[macro_use] extern crate log;
#[macro_use] extern crate clap;
extern crate loggerv;

use clap::App;

fn get_path_vector<'a>(app : &'a clap::ArgMatches) -> Vec<&'a str>
{
    match app.values_of("paths") {
        None => vec!["."],
        Some(t) => t.collect(),
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let params = App::from_yaml(yaml).get_matches();
    loggerv::init_with_verbosity(params.occurrences_of("verbose")).unwrap();
    warn!("verbosity = {}", params.occurrences_of("verbose"));

    info!("Path parameters:");
    let paths = get_path_vector(&params);
    for path in paths {
        info!("  - {}", path);
    }
}
