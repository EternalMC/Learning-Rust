extern crate num_cpus;

mod simple;
mod loops;
mod run;
// 325dfa5 is the commit
fn main() {
    let num = num_cpus::get();
    println!("Number of Cores: {}", num);
    simple::run();
    simple::math();
    loops::loops();
    loops::ifloop();
    run::main();
}
