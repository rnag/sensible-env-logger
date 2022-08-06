#![doc(html_root_url = "https://docs.rs/sensible-env-logger/0.3")]
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
//! A simple logger, optionally configured via environment variables which
//! writes to standard error with nice colored output for log levels.
//! It sets up logging with "sensible" defaults that make it ideal for
//! running *[examples]* and *[tests]* on a crate of choice.
//!
//! [examples]: http://xion.io/post/code/rust-examples.html
//! [tests]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
//! <br>
//!
//! ## Usage
//!
//! Even though it has `env` in the name, the `sensible-env-logger`
//! requires minimal configuration and setup to use:
//!
//! ```
//! #[macro_use] extern crate log;
//!
//! fn main() {
//!     sensible_env_logger::init!();
//!
//!     trace!("a trace example");
//!     debug!("deboogging");
//!     info!("such information");
//!     warn!("o_O");
//!     error!("boom");
//! }
//! ```
//!
//! Run the program and you should see all the log output for your crate.
//!
//! Alternatively, run the program with the environment variables that control
//! the log level for *your* crate as well as *external* crates explicitly set,
//! like `RUST_LOG=debug` and `GLOBAL_RUST_LOG=error`.
//!
//! ## Defaults
//!
//! The defaults can be setup by calling `init!()` or `try_init!()` at the start
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
#[doc(hidden)]
pub use pretty_env_logger as pretty;
#[doc(hidden)]
pub use pretty_env_logger::env_logger as env;

use std::borrow::Cow;

use env::Builder;
use log::{trace, SetLoggerError};

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
/// This macro fails to set the global logger if one has already been set.
#[macro_export]
macro_rules! init {
    () => {
        $crate::try_init!().unwrap()
    };
}

/// Initializes the global logger with a timed pretty, sensible env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This macro fails to set the global logger if one has already been set.
#[macro_export]
macro_rules! init_timed {
    () => {
        $crate::try_init_timed!().unwrap();
    };
}

/// Initializes the global logger with a pretty, sensible env logger.
///
/// This variant should ideally only be used in **tests**. It should be called
/// early in the execution of a Rust program.
///
/// Future initialization attempts will *safely ignore* any errors.
#[macro_export]
macro_rules! safe_init {
    () => {
        let _ = $crate::try_init!();
    };
}

/// Initializes the global logger with a timed pretty, sensible env logger.
///
/// This variant should ideally only be used in **tests**. It should be called
/// early in the execution of a Rust program.
///
/// Future initialization attempts will *safely ignore* any errors.
#[macro_export]
macro_rules! safe_init_timed {
    () => {
        let _ = $crate::try_init_timed!();
    };
}

/// Initializes the global logger with a pretty, sensible env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This macro fails to set the global logger if one has already been set.
#[macro_export]
macro_rules! try_init {
    () => {
        $crate::try_init_custom_env_and_builder(
            "RUST_LOG",
            "GLOBAL_RUST_LOG",
            env!("CARGO_PKG_NAME"),
            module_path!(),
            $crate::pretty::formatted_builder,
        )
    };
}

/// Initializes the global logger with a timed pretty, sensible env logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This macro fails to set the global logger if one has already been set.
#[macro_export]
macro_rules! try_init_timed {
    () => {
        $crate::try_init_custom_env_and_builder(
            "RUST_LOG",
            "GLOBAL_RUST_LOG",
            env!("CARGO_PKG_NAME"),
            module_path!(),
            $crate::pretty::formatted_timed_builder,
        )
    };
}

/// Initializes the global logger with a pretty, sensible env logger, with custom
/// variable names and a custom builder function.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Example
/// ```rust
/// sensible_env_logger::try_init_custom_env_and_builder(
///     "MY_RUST_LOG",
///     "MY_GLOBAL_RUST_LOG",
///     env!("CARGO_PKG_NAME"),
///     module_path!(),
///     sensible_env_logger::pretty::formatted_timed_builder,
/// );
/// ```
///
/// # How It works
///
/// The `package_name` and `module_name` arguments are ideally evaluated from
/// the `$CARGO_PKG_NAME` and `$CARGO_CRATE_NAME` environment variables
/// respectively. These environment variables are automatically set
/// by Cargo when compiling your crate. It then builds a custom directives
/// string in the same form as the `$RUST_LOG` environment variable, and then
/// parses this generated directives string using
/// `env_logger::Builder::parse_filters`.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_custom_env_and_builder(
    log_env_var: &str,
    global_log_env_var: &str,
    package_name: &str,
    module_name: &str,
    builder_fn: impl Fn() -> Builder,
) -> Result<(), SetLoggerError> {
    let package_name = package_name.replace('-', "_");
    let module_name = base_module(module_name);

    let log_level = get_env(log_env_var, CRATE_LOG_LEVEL);
    let global_log_level = get_env(global_log_env_var, GLOBAL_LOG_LEVEL);

    let filters_str = if log_level.contains('=') {
        // The env variable `$RUST_LOG` is set to a more complex value such as
        // `warn,my_module=info`. In that case, just pass through the value.
        log_level.into_owned()
    } else if package_name != module_name {
        format!(
            "{default_lvl},{pkg}={lvl},{mod}={lvl}",
            default_lvl = global_log_level,
            pkg = package_name,
            mod = module_name,
            lvl = log_level
        )
    } else {
        format!(
            "{default_lvl},{pkg}={lvl}",
            default_lvl = global_log_level,
            pkg = package_name,
            lvl = log_level
        )
    };

    let mut builder: Builder = builder_fn();
    builder.parse_filters(&filters_str);

    let result = builder.try_init();

    trace!("Filter: {}", filters_str);

    result
}

/// Retrieve the value of an environment variable.
pub(crate) fn get_env<'a>(env_var_name: &'a str, default: &'a str) -> Cow<'a, str> {
    match std::env::var(env_var_name) {
        Ok(value) => Cow::Owned(value),
        _ => Cow::Borrowed(default),
    }
}

/// Returns the base module name, given the path to a module.
///
/// # Example
/// ```no_test
/// assert_eq!(base_module("my_bin::my_module::tests"), "my_bin");
/// ```
///
pub(crate) fn base_module(module_name: &str) -> &str {
    match module_name.split_once("::") {
        Some((first, _)) => first,
        None => module_name,
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

    /// Local time zone format (only time)
    ///
    /// # Example
    /// `10:17:52.831`
    ///
    pub const TIME_ONLY_FMT: &str = "%l:%M:%S.%3f";

    /// Local time zone format
    ///
    /// # Example
    /// `2022-07-27 10:17:52.831 -`
    ///
    pub const LOCAL_TIME_FMT: &str = "%Y-%m-%d %l:%M:%S.%3f -";

    /// ISO 8601 / RFC 3339 date & time format.
    ///
    /// # Example
    /// `2022-07-27T17:34:44.531+08:00`
    ///
    pub const ISO_FMT: &str = "%Y-%m-%dT%H:%M:%S.%3f%:z";

    /// Initializes the global logger with an "abbreviated" timed pretty, sensible
    /// env logger.
    ///
    /// This should be called early in the execution of a Rust program, and the
    /// global logger may only be initialized once. Future initialization attempts
    /// will return an error.
    ///
    /// # Details
    ///
    /// This variant formats log messages with a localized timestamp, without
    /// the date part.
    ///
    /// ## Example
    ///
    /// ```console
    /// 12:15:31.683 INFO  my_module         > an info message!
    /// ```
    ///
    /// # Requirements
    ///
    /// Using this macro requires the `local-time` feature to be enabled:
    ///
    /// ```toml
    /// [dependencies]
    /// sensible-env-logger = { version = "*", features = ["local-time"] }
    /// ```
    ///
    /// # Panics
    ///
    /// This macro fails to set the global logger if one has already been set.
    #[macro_export]
    macro_rules! init_timed_short {
        () => {
            $crate::try_init_timed_short!().unwrap();
        };
    }

    /// Initializes the global logger with a "no-frills" local date/time
    /// pretty, sensible env logger.
    ///
    /// This should be called early in the execution of a Rust program, and the
    /// global logger may only be initialized once. Future initialization attempts
    /// will return an error.
    ///
    /// # Details
    ///
    /// This variant formats log messages with a localized timestamp,
    /// prefixed by the date part.
    ///
    /// ## Example
    ///
    /// ```console
    /// 2021-10-27 12:15:31.683 - INFO  my_module         > an info message!
    /// ```
    ///
    /// # Requirements
    ///
    /// Using this macro requires the `local-time` feature to be enabled:
    ///
    /// ```toml
    /// [dependencies]
    /// sensible-env-logger = { version = "*", features = ["local-time"] }
    /// ```
    ///
    /// # Panics
    ///
    /// This macro fails to set the global logger if one has already been set.
    #[macro_export]
    macro_rules! init_timed_local {
        () => {
            $crate::try_init_timed_local!().unwrap();
        };
    }

    /// Initializes the global logger with a local-timed pretty, sensible
    /// env logger.
    ///
    /// This should be called early in the execution of a Rust program, and the
    /// global logger may only be initialized once. Future initialization attempts
    /// will return an error.
    ///
    /// # Details
    ///
    /// This variant formats log messages with a localized timestamp and zone,
    /// in complete ISO-8601/ RFC 3339 date & time format.
    ///
    /// ## Example
    ///
    /// ```console
    /// 2022-10-27T12:15:31.683+08:00 - INFO  my_module         > an info message!
    /// ```
    ///
    /// # Requirements
    ///
    /// Using this macro requires the `local-time` feature to be enabled:
    ///
    /// ```toml
    /// [dependencies]
    /// sensible-env-logger = { version = "*", features = ["local-time"] }
    /// ```
    ///
    /// # Panics
    ///
    /// This macro fails to set the global logger if one has already been set.
    #[macro_export]
    macro_rules! init_timed_local_iso {
        () => {
            $crate::try_init_timed_local_iso!().unwrap();
        };
    }

    /// Initializes the global logger with an "abbreviated" timed pretty, sensible
    /// env logger.
    /// See [`init_timed_short!`](macro.init_timed_short.html) for more info.
    ///
    /// This variant should ideally only be used in **tests**. It should be called
    /// early in the execution of a Rust program.
    ///
    /// Future initialization attempts will *safely ignore* any errors.
    #[macro_export]
    macro_rules! safe_init_timed_short {
        () => {
            let _ = $crate::try_init_timed_short!();
        };
    }

    /// Initializes the global logger with a "no-frills" local date/time
    /// pretty, sensible env logger.
    /// See [`init_timed_local!`](macro.init_timed_local.html) for more info.
    ///
    /// This variant should ideally only be used in **tests**. It should be called
    /// early in the execution of a Rust program.
    ///
    /// Future initialization attempts will *safely ignore* any errors.
    #[macro_export]
    macro_rules! safe_init_timed_local {
        () => {
            let _ = $crate::try_init_timed_local!();
        };
    }

    /// Initializes the global logger with a local-timed pretty, sensible
    /// env logger.
    /// See [`init_timed_local_iso!`](macro.init_timed_local_iso.html) for more info.
    ///
    /// This variant should ideally only be used in **tests**. It should be called
    /// early in the execution of a Rust program.
    ///
    /// Future initialization attempts will *safely ignore* any errors.
    #[macro_export]
    macro_rules! safe_init_timed_local_iso {
        () => {
            let _ = $crate::try_init_timed_local_iso!();
        };
    }

    /// Initializes the global logger with an "abbreviated" timed pretty, sensible
    /// env logger.
    ///
    /// See [`init_timed_short!`](macro.init_timed_short.html) for more info.
    ///
    /// This should be called early in the execution of a Rust program, and the
    /// global logger may only be initialized once. Future initialization attempts
    /// will return an error.
    ///
    /// # Errors
    ///
    /// This macro fails to set the global logger if one has already been set.
    #[macro_export]
    macro_rules! try_init_timed_short {
        () => {
            $crate::try_init_custom_env_and_builder(
                "RUST_LOG",
                "GLOBAL_RUST_LOG",
                env!("CARGO_PKG_NAME"),
                module_path!(),
                $crate::formatted_local_time_builder_fn($crate::TIME_ONLY_FMT),
            )
        };
    }

    /// Initializes the global logger with a "no-frills" local date/time
    /// pretty, sensible env logger.
    ///
    /// See [`init_timed_local!`](macro.init_timed_local.html) for more info.
    ///
    /// This should be called early in the execution of a Rust program, and the
    /// global logger may only be initialized once. Future initialization attempts
    /// will return an error.
    ///
    /// # Errors
    ///
    /// This macro fails to set the global logger if one has already been set.
    #[macro_export]
    macro_rules! try_init_timed_local {
        () => {
            $crate::try_init_custom_env_and_builder(
                "RUST_LOG",
                "GLOBAL_RUST_LOG",
                env!("CARGO_PKG_NAME"),
                module_path!(),
                $crate::formatted_local_time_builder_fn($crate::LOCAL_TIME_FMT),
            )
        };
    }

    /// Initializes the global logger with a local-timed pretty, sensible
    /// env logger.
    ///
    /// See [`init_timed_local_iso!`](macro.init_timed_local_iso.html) for more info.
    ///
    /// This should be called early in the execution of a Rust program, and the
    /// global logger may only be initialized once. Future initialization attempts
    /// will return an error.
    ///
    /// # Errors
    ///
    /// This macro fails to set the global logger if one has already been set.
    #[macro_export]
    macro_rules! try_init_timed_local_iso {
        () => {
            $crate::try_init_custom_env_and_builder(
                "RUST_LOG",
                "GLOBAL_RUST_LOG",
                env!("CARGO_PKG_NAME"),
                module_path!(),
                $crate::formatted_local_time_builder_fn($crate::ISO_FMT),
            )
        };
    }

    /// Returns a function (closure) that returns a formatted builder
    /// function which adds local time to log messages, per a specified
    /// date/time format.
    ///
    /// ## Example
    /// ```console
    /// 12:15:31.683 INFO  my_module         > an info message!
    /// ```
    ///
    pub fn formatted_local_time_builder_fn(fmt: &'static str) -> impl Fn() -> Builder {
        || {
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

                let time = Local::now().format(fmt);

                writeln!(f, " {} {} {} > {}", time, level, target, record.args(),)
            });

            builder
        }
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
    use log::*;

    #[test]
    fn logging_in_tests() {
        // Initialize the global logger with sensible defaults
        init!();

        trace!("A simple trace message");
        debug!("Debugging something...");
        warn!("This is a WARNING!");
    }

    #[test]
    fn test_base_module_simple() {
        let result = base_module("hello_world");
        assert_eq!(result, "hello_world");
    }

    #[test]
    fn test_base_module_with_nested_path() {
        let result = base_module("my_bin::my_module::tests");
        assert_eq!(result, "my_bin");
    }
}
