#[macro_use]
extern crate log;

mod nested {
    pub fn deep() {
        trace!("one level deep!");
    }
}

fn main() {
    sensible_env_logger::init!();
    nested::deep();
    info!("Run `cargo test` to see the complete output!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        sensible_env_logger::safe_init!();

        nested::deep();

        debug!("deboogging");
        info!("such information");
        warn!("o_O");
        error!("boom");
    }
}
