use cmd_grep::*;
use std::env;

fn main() {
    let conf = Conf::build(env::args()).unwrap();
    run(conf);
}
