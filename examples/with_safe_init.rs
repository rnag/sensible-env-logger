#[macro_use]
extern crate log;

mod one {
    pub fn deep() {
        sensible_env_logger::safe_init!();
        trace!("one level deep!");
    }
}

fn main() {
    sensible_env_logger::safe_init!();

    info!("such information");
    warn!("o_O");
    error!("boom");
    debug!("deboogging");
    self::one::deep();
}
