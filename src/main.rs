#![allow(non_snake_case)]

use structopt::StructOpt;

mod lexer;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "repL", long)]
    repl: bool,

    #[structopt(name = "print-ir", long)]
    print_ir: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
