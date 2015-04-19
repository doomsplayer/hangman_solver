# Usage: 

1. Download and install `rustc` nightly (not tested in beta.2) [rust](http://www.rust-lang.org/)

2. Install [cargo](https://github.com/rust-lang/cargo). 

3. If you have got a archlinux, you can found rust-nightly-bin in AUR with both rustc and cargo archived.

3. `# cargo run --example resolver guess -d path/to/dict` will start the brute force guess.

4. `cargo run --example guesser` is the guesser. It can be used offline, in any hangman-like game.

5. preprocess is the pre-process program for formatting the dict file. don't touch it.

6. there is a dummy timeout mechanism in httpconnector.rs since rust-lang is in developing status and timeout 
for socket will be shipped in rust 1.1 which won't be released before July. So I write an ffi to use C's 
socket, but still buggy. I submitted a pull request to rust compile yesterday to add this timeout feature, 
waiting for merge.
