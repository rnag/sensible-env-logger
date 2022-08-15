extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use sensible_env_logger::TIME_ONLY_FMT;
use sensible_env_logger::env::Target;

mod one {
    pub fn deep() {
        trace!("one level deep!");
        trace!("one level deep!");
    }
}

fn main() {
    sensible_env_logger::formatted_local_time_builder_fn(TIME_ONLY_FMT)()
        //let's just set some random stuff.. for more see
        //https://docs.rs/env_logger/0.5.0-rc.1/env_logger/struct.Builder.html
        .target(Target::Stdout)
        .parse_filters("with_builder=trace")
        .init();

    info!("such information");
    info!("such information");
    warn!("o_O");
    warn!("o_O");
    error!("boom");
    error!("boom");
    debug!("deboogging");
    debug!("deboogging");
    self::one::deep();
}
