#![feature(plugin)]
#![plugin(docopt_macros)]
extern crate docopt;
extern crate rustc_serialize;
extern crate hangman;
use hangman::*;

docopt!(Args, r"
Usage: guesser guess <word> -h <history> -d <dict>

Options:
        -h <history>  Guess history
        -d <dict>     Path to dict file.
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit()); // parsing arguments
    
    let d = Dict::new(&args.flag_d).unwrap(); // new a dict to guess
    let word = args.arg_word;
    let history = args.flag_h;
    let mut guesser = d.guess();
    guesser.set_history(history.chars().collect());
    let a_guess = guesser.guess(&word);
    println!("{:?}", a_guess);
}
