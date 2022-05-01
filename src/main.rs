use std::fs::File;
use std::io::BufReader;

use gpx::read;
use gpx::Gpx;

mod plotting;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/sample.png";

fn main() {
    // This XML file actually exists — try it for yourself!
    let file = File::open("test.GPX").unwrap();
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let gpx: Gpx = read(reader).unwrap();

    plotting::create_plot(&gpx, OUT_FILE_NAME).unwrap();
}
