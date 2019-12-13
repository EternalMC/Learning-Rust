use crate::main;
use crate::run::complex::main;

mod simple;
mod loops;
mod run;

fn main() {
    simple::run();
    simple::math();
    loops::loops();
    loops::ifloop();
    run::main();
}
