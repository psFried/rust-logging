#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate test_logger;

extern crate log4rs;
extern crate log4rs_rolling_file;

use std::thread;
use std::time::Duration;

pub fn log_some_stuff() {
    trace!("This is a trace statement");
    debug!("This is a debug statement");
    info!("This is an info statement");
    warn!("This is a warn statement");
    error!("This is an error statement");
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
    loop {
        log_some_stuff();
        thread::sleep(Duration::new(1, 0));
    }
}


#[cfg(test)]
mod test {
    use test_logger;
    use super::*;

    test!{log_output_prints_when_this_test_fails, {
        log_some_stuff();
        panic!("Holy guacamole!");
    }}

    test!{no_log_output_here_because_this_test_passes, {
        info!("Might not see this unless you run tests using `cargo test -- --nocapture`")
        // Caveat - log output doesn't always get captured perfectly since it is written by a background thread
    }}
}


