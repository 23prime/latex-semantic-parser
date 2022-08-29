use std::{env, process::exit};

use log::{error, info};

use latex_equivalencer::exec;
use latex_equivalencer::logger;

fn main() {
    logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        error!("Give just 2 arguments except for `target/*`.");
        exit(1);
    }

    let lhs = &args[1];
    let rhs = &args[2];
    info!("Inputs => {{ lhs => {:?}, rhs => {:?} }}", lhs, rhs);

    let result = exec(lhs, rhs);
    info!("Result => {:?}", result);
}
