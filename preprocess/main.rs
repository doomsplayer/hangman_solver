extern crate encoding;
use std::fs::File;
use std::error::Error;
use std::io::Read;
use encoding::DecoderTrap;
use encoding::all::WINDOWS_1251;
use encoding::Encoding;
use std::ascii::AsciiExt;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;

fn main() {
    doit("/home/wooya/Downloads/scrow/eng3").unwrap();
}

fn doit(path: &str) -> Result<(), Box<Error>> {
    let mut f = try!(OpenOptions::new().read(true).write(true).open(path));
    let mut v = vec![];
    f.read_to_end(&mut v);
    let mut s = String::new();
    let ret = WINDOWS_1251.decode_to(&v[..], DecoderTrap::Strict, &mut s);

    let a: Vec<String> = s.split("\n").filter_map(
        |x| {
            if x.is_ascii() {
                Some(x.to_ascii_uppercase())
            } else {
                None
            }
        }).collect();
    let s = a.into_iter().fold(String::new(), |mut acc, s| {acc.push('\n'); acc.push_str(&s[..]); acc});
    try!(f.set_len(0));
    try!(f.seek(SeekFrom::Start(0)));
    let _ = write!(f, "{}", s);
    Ok(())
}

