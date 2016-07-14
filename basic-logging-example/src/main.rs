#[macro_use]
extern crate log;
extern crate env_logger;

#[cfg(test)]
#[macro_use]
extern crate test_logger;

mod my_mod;

fn main() {
    env_logger::init().unwrap();

    trace!("This is the finest level of log statement");
    debug!("This is intended to be slightly less granular");
    info!("This is intended for higher-level events");
    warn!("This could indicate an issue, but is still disabled by default");
    error!("This is the only log level that is enabled by default");

    my_mod::do_stuff();
}

