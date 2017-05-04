extern crate schani;

use schani::core::images::RawtherapeeImage;
use schani::processing::process_raw;

fn main() {
    let image = RawtherapeeImage {
        name: "hello".to_string(),
        raw: "resources/DSC_2936.NEF".to_string(),
        sidecar: "resources/DSC_2936.NEF.pp3".to_string(),
    };
    process_raw(&image);
}
