use pingora::prelude::Opt;
use structopt::StructOpt;

use proksis::server;

fn main() {
    println!("Hello, world!");
    let opt = Some(Opt::from_args());
    server::run(opt);
}
