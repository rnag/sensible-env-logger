# Examples

This folder contains example scripts that can be used to interact with
the `sensible-env-logger` crate.

## Quickstart

[cargo-rx]: https://github.com/rnag/cargo-rx

Install my crate [cargo-rx], which abstracts away `cargo run --example`.
This provides a single `rx` command.

```shell
❯❯ cargo install cargo-rx
```

Now start out by cloning the GitHub project:

```shell
❯❯ git clone https://github.com/rnag/sensible-env-logger.git
```

Then, simply just `cd` into the project folder:

```shell
❯❯ cd sensible-env-logger
```

From here, you can use `rx` to build and run
any of the examples individually.

In particular, here's a sample usage of running `examples/log.rs`:

```shell
❯❯ rx log
```

If you run the command without any arguments, you can select
from the list of available examples:

```shell
❯❯ rx
```

An example of including the local timestamp (without the date part) in the log
messages:

```shell
>> rx log_timed_short
```

To pass arguments to a script, you can include them after the `--`.

For example, here's an example of passing arguments to the `log` script:

```shell
❯❯ rx log -- \
     -n "My Name" \
     -d "My description"
```

## Setting the Log Level

While not necessary, you can also explicitly set the log level for the *example*
as well as the *library* under test, `sensible-env-logger` in this case.

Therefore, remember to ensure that the **RUST_LOG** env variable
is properly set. You can also optionally set the **GLOBAL_RUST_LOG** env
variable, which sets the default log level for external crates and libraries.

For example, on *Mac/Linux*:

```shell
❯❯ export RUST_LOG='TRACE'
❯❯ export GLOBAL_RUST_LOG='WARN'
```

On *Windows*:

```shell
❯❯ $env:RUST_LOG='TRACE'
❯❯ $env:GLOBAL_RUST_LOG='WARN'
```
