// noinspection DuplicatedCode
#[macro_use]
extern crate log;

mod nested {
    pub fn deep() {
        trace!("one level deep!");
    }
}

fn main() {
    sensible_env_logger::init_timed_short!();

    // generally, we shouldn't get here actually
    if !log_enabled!(log::Level::Trace) {
        eprintln!("To see the full demo, try setting `RUST_LOG=trace`.");
        eprintln!();
    }

    self::nested::deep();
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}
