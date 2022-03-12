#![doc(html_root_url = "https://docs.rs/sensible-env-logger/0.0.1")]
#![warn(rust_2018_idioms, missing_docs)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]

//! [![github]](https://github.com/rnag/sensible-env-logger)&ensp;[![crates-io]](https://crates.io/crates/sensible-env-logger)&ensp;[![docs-rs]](https://docs.rs/sensible-env-logger)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! A pretty, sensible logger for Rust.
//!
//! <br>
//!
//! ## Usage
//!
//! Even though it has `env` in the name, the `sensible-env-logger`
//! requires zero configuration and setup to use:
//!
//! ```
//! use log::*;
//!
//! fn main() {
//!     sensible_env_logger::init();
//!
//!     trace!("a trace example");
//!     debug!("deboogging");
//!     info!("such information");
//!     warn!("o_O");
//!     error!("boom");
//! }
//! ```
//!
//! ## Defaults
//!
//! The defaults can be setup by calling `init()` or `try_init()` at the start
//! of the program.
//!
//! ## Examples
//!
//! You can check out sample usage of this crate in the [examples/](https://github.com/rnag/sensible-env-logger/tree/main/examples)
//! folder in the project repo on GitHub.
//!
//! ## Readme Docs
//!
//! You can find the crate's readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! [crates.io]: https://crates.io/crates/sensible-env-logger
//! [`README.md`]: https://github.com/rnag/sensible-env-logger
//!
//! ## Enable logging
//!
//! This crate uses [pretty_env_logger] and [env_logger] internally, so the
//! same ways of enabling logs through an environment variable are supported.
//!
//! The `sensible_env_logger` crate re-exports these crates, through the
//! `pretty` and `env` namespaces respectively.
//!
//! [pretty_env_logger]: https://docs.rs/pretty_env_logger
//! [env_logger]: https://docs.rs/env_logger

#[cfg(feature = "local-time")]
pub use local_time::*;
pub use pretty_env_logger as pretty;
pub use pretty_env_logger::env_logger as env;

use std::borrow::Cow;
use std::path::Path;

use env::Builder;
use log::SetLoggerError;

/// Default log level for the Cargo crate or package under test.
pub(crate) const CRATE_LOG_LEVEL: &str = "trace";

/// Default log level for external crates, other than the one under test.
pub(crate) const GLOBAL_LOG_LEVEL: &str = "warn";

/// Initializes the global logger with a pretty, sensible env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
#[track_caller]
pub fn init() {
    try_init().unwrap()
}

/// Initializes the global logger with a timed pretty, sensible env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
#[track_caller]
pub fn init_timed() {
    try_init_timed().unwrap();
}

/// Initializes the global logger with a pretty, sensible env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
#[track_caller]
pub fn try_init() -> Result<(), SetLoggerError> {
    try_init_custom_env_and_builder(
        "RUST_LOG",
        "GLOBAL_RUST_LOG",
        pretty_env_logger::formatted_builder,
    )
}

/// Initializes the global logger with a timed pretty, sensible env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
#[track_caller]
pub fn try_init_timed() -> Result<(), SetLoggerError> {
    try_init_custom_env_and_builder(
        "RUST_LOG",
        "GLOBAL_RUST_LOG",
        pretty_env_logger::formatted_timed_builder,
    )
}

/// Initialized the global logger with a pretty, sensible env logger, with custom
/// variable names and a custom builder function.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # How It works
///
/// The package name is automatically taken from the `$CARGO_CRATE_NAME`
/// environment variable. This environment variable is automatically set
/// by Cargo when compiling your crate. It then builds a custom directives
/// string in the same form as the `$RUST_LOG` environment variable, and then
/// parses this generated directives string using
/// `env_logger::Builder::parse_filters`.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
#[track_caller]
pub fn try_init_custom_env_and_builder(
    log_env_var: &str,
    global_log_env_var: &str,
    builder_fn: impl Fn() -> Builder,
) -> Result<(), SetLoggerError> {
    let file_path = std::panic::Location::caller().file();

    let module_name = match Path::new(file_path).file_stem() {
        Some(stem) => stem.to_str().unwrap(),
        None => file_path,
    };

    let crate_name = env!("CARGO_CRATE_NAME");

    let log_level = get_env(log_env_var, CRATE_LOG_LEVEL);
    let global_log_level = get_env(global_log_env_var, GLOBAL_LOG_LEVEL);

    let filters_str = format!(
        "{default_lvl},{pkg}={lvl},{mod}={lvl}",
        default_lvl = global_log_level,
        pkg = crate_name,
        mod = module_name,
        lvl = log_level
    );

    let mut builder: Builder = builder_fn();
    builder.parse_filters(&filters_str);
    builder.try_init()
}

/// Retrieve the value of an environment variable.
pub(crate) fn get_env<'a>(env_var_name: &'a str, default: &'a str) -> Cow<'a, str> {
    match std::env::var(env_var_name) {
        Ok(value) => Cow::from(value),
        _ => Cow::Borrowed(default),
    }
}

#[cfg(feature = "local-time")]
mod local_time {
    use super::*;

    use std::fmt;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use chrono::Local;
    use env::fmt::{Color, Style, StyledValue};
    use log::Level;

    /// Initializes the global logger with an "abbreviated" timed pretty, sensible
    /// env logger.
    ///
    /// This should be called early in the execution of a Rust program, and the
    /// global logger may only be initialized once. Future initialization attempts
    /// will return an error.
    ///
    /// # About
    ///
    /// This variant prints log messages with a localized timestamp, without
    /// the date part.
    ///
    /// ## Example
    /// ```console
    /// 12:15:31.683 INFO  my_module         > an info message!
    /// ```
    ///
    /// # Panics
    ///
    /// This function fails to set the global logger if one has already been set.
    #[track_caller]
    pub fn init_timed_short() {
        try_init_timed_short().unwrap();
    }

    /// Initializes the global logger with an "abbreviated" timed pretty, sensible
    /// env logger.
    ///
    /// This should be called early in the execution of a Rust program, and the
    /// global logger may only be initialized once. Future initialization attempts
    /// will return an error.
    ///
    /// # About
    ///
    /// This variant prints log messages with a localized timestamp, without
    /// the date part.
    ///
    /// ## Example
    /// ```console
    /// 12:15:31.683 INFO  my_module         > an info message!
    /// ```
    ///
    /// # Errors
    ///
    /// This function fails to set the global logger if one has already been set.
    #[track_caller]
    pub fn try_init_timed_short() -> Result<(), SetLoggerError> {
        try_init_custom_env_and_builder(
            "RUST_LOG",
            "GLOBAL_RUST_LOG",
            formatted_short_timed_builder,
        )
    }

    /// Returns a formatted builder which adds local time to log messages.
    ///
    /// ## Example
    /// ```console
    /// 12:15:31.683 INFO  my_module         > an info message!
    /// ```
    ///
    pub fn formatted_short_timed_builder() -> Builder {
        let mut builder = Builder::new();

        builder.format(|f, record| {
            use std::io::Write;
            let target = record.target();
            let max_width = max_target_width(target);

            let mut style = f.style();
            let level = colored_level(&mut style, record.level());

            let mut style = f.style();
            let target = style.set_bold(true).value(Padded {
                value: target,
                width: max_width,
            });

            let time = Local::now().format("%l:%M:%S.%3f");

            writeln!(f, " {} {} {} > {}", time, level, target, record.args(),)
        });

        builder
    }

    /// Helper functions
    ///
    /// Below are copied verbatim from [`pretty_env_logger`]
    ///
    /// [`pretty_env_logger`]: https://github.com/seanmonstar/pretty-env-logger/blob/master/src/lib.rs
    ///

    struct Padded<T> {
        value: T,
        width: usize,
    }

    impl<T: fmt::Display> fmt::Display for Padded<T> {
        fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
            write!(f, "{: <width$}", self.value, width = self.width)
        }
    }

    static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

    fn max_target_width(target: &str) -> usize {
        let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
        if max_width < target.len() {
            MAX_MODULE_WIDTH.store(target.len(), Ordering::Relaxed);
            target.len()
        } else {
            max_width
        }
    }

    fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
        match level {
            Level::Trace => style.set_color(Color::Magenta).value("TRACE"),
            Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
            Level::Info => style.set_color(Color::Green).value("INFO "),
            Level::Warn => style.set_color(Color::Yellow).value("WARN "),
            Level::Error => style.set_color(Color::Red).value("ERROR"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, trace, warn};

    #[test]
    fn logging_in_tests() {
        // Initialize the global logger with sensible defaults
        init();

        trace!("A simple trace message");
        debug!("Debugging something...");
        warn!("This is a WARNING!");
    }
}
