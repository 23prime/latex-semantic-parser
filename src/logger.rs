use std::env;
use std::io::Write;

use chrono::Local;
use env_logger::{Builder, Env};

pub fn init() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }

    let env = Env::default().filter("RUST_LOG");
    let _ = Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .try_init();
}
