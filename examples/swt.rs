extern crate ccv;

use std::env::args;

use ccv::*;
use ccv::swt::*;

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();        // Unused: executable name.
    let file_in = args.next().expect("Expected an input file");
    let file_out = args.next().expect("Expected an output file");

    println!("Attempting to open \"{}\"", file_in);
    let mut pix = Matrix::read(file_in, OpenAs::ToGray).expect("Could not read image");

    println!("Attempting to swt-convert");
    let pix2 = pix.swt(SwtParams::default());

    println!("Attempting to swt-write");
    pix2.write(file_out, FileFormat::PNG).expect("Could not write image");
    println!("Done");
}