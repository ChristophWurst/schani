use core::images::RawtherapeeImage;

use std::process::Command;

pub fn process_raw(img: &RawtherapeeImage) {
    // rawtherapee -j90 -Y -c resources/DSC_2936.NEF
    let out = Command::new("./convert.sh")
        .arg(&img.raw)
        .output()
        .expect("failed to start Rawtherapee");

    println!("std output: {}", String::from_utf8_lossy(&out.stdout));
    println!("err output: {}", String::from_utf8_lossy(&out.stderr));
}
