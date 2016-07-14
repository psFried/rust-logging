Logging in Rust
===============

This repository is here to provide some examples of logging in Rust. There are two separate rust projects. The first is a basic example using the `env_logger` crate. It is designed to show the basics of how the `log` crate works. The second example, `file-logging` is there to provide an example of working with a more advanced logging backend.

## Basics of logging with Rust

- Use the [log crate](https://crates.io/crates/log)
    - provides a backend-agnostic logging interface
    - Really easy to use different backends in different scenarios
    - defaults to a no-op backend with a really low overhead
- The [env_logger](https://crates.io/crates/env_logger) crate is _by far_ the most popular logging backend
    - Just prints to stdout
    - Can set different log levels by setting the `RUST_LOG` environment variable
    - Built in filtering of log messages using regex
    - Can customize the format using a closure to format log messages
    - Can customize the name of the environment variable that's used for filtering
    - Best choice for use in tests
- [log4rs](https://crates.io/crates/log4rs) is a good choice if you need to log to files
    - Basically a Rust clone of log4j
    - Configuration can be done either in code or using one of several file formats
    - Can handle all the crazy logging configurations you're likely to need
    - Extensible, can write your own appender if you want
    - The only Rust logging library that has a rolling file appender as far as I can tell
- [Fern](https://crates.io/crates/fern) is also somewhat popular, but I have not used it myself
    - Appears nearly on par with lo4rs in terms of features
    - No commits in over a year :(
