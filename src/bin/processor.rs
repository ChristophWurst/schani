extern crate schani;

use schani::core::images::RawImage;
use schani::processing::process_raw;

fn main() {
    println!("hello from processor!");

    let x = RawImage { name: "hello".to_string() };
    process_raw(&x);
}
