#[macro_use] extern crate log;
extern crate loggerv;

fn main() {
    loggerv::init_with_verbosity(1).unwrap();
    info!("Hello world (info)");
    warn!("Hello world (warn)");
    error!("Hello world (error)");
}
