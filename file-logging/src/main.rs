#[macro_use]
extern crate log;

extern crate log4rs;
extern crate log4rs_rolling_file;

use std::thread;
use std::time::Duration;

fn log_some_stuff() {
    loop {
        trace!("This is a trace statement");
        debug!("This is a debug statement");
        info!("This is an info statement");
        warn!("This is a warn statement");
        error!("This is an error statement");
        thread::sleep(Duration::new(1, 0));
    }
}

fn init_logger() {
    use log4rs::init_file;
    use log4rs::file::Deserializers;

    let mut deserializers = Deserializers::default();
    log4rs_rolling_file::register(&mut deserializers);
    init_file("logging-configuration.yaml", deserializers).unwrap();
}

fn main() {
    init_logger();
    log_some_stuff();
}
