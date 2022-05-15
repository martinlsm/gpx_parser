use std::fs::File;
use std::io::BufReader;

use gpx::read;
use gpx::Gpx;

mod error;
mod plotting;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/lundaloppet.png";

fn main() {
    // This XML file actually exists — try it for yourself!
    let file = File::open("lundaloppet-2022-babel.gpx").unwrap();
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let mut gpx: Gpx = read(reader).unwrap();

    plotting::create_plot(&mut gpx, OUT_FILE_NAME).unwrap();
}
