extern crate crypto;

use std::io;
use std::io::prelude::*;
use std::fs::File;

let mut counter = 0;

fn rand_id() -> Int {
	let mut csprng = try!(File::open("/dev/urandom"))
	let mut buffer = [0; 20];

	try!(csprng.read(&mut buffer));

	let mut buffer = vec![0; 20];
	try!(f.read_to_end(&mut buffer));

	// the rest is WIP :P
}
