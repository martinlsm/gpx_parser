use std::fs::File;
use std::io::BufReader;

use gpx::read;
use gpx::{Gpx, Track, TrackSegment};

use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/sample.png";

fn main() {
    // This XML file actually exists — try it for yourself!
    let file = File::open("test.GPX").unwrap();
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let gpx: Gpx = read(reader).unwrap();

    // Each GPX file has multiple "tracks", this takes the first one.
    let track: &Track = &gpx.tracks[0];
    match &track.name {
        Some(n) => println!("Track name: {}", n),
        None => println!("Track has no name"),
    }

    // Each track will have different segments full of waypoints, where a
    // waypoint contains info like latitude, longitude, and elevation.
    let segment: &TrackSegment = &track.segments[0];
    let num_points: usize = segment.points.len();
    let elevation: Vec<f64> = (0..num_points)
        .map(|i| segment.points[i].elevation.unwrap())
        .collect();
    let speed: Vec<f64> = (0..num_points)
        .map(|i| segment.points[i].speed.unwrap_or(0.0))
        .collect();

    println!("Num points: {}", num_points);

    println!("p[0]: {:?}", segment.points[0].point());

    let drawing_area = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..num_points, -300.0..300.0)
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..num_points).map(|i| (i, elevation[i])),
            &RED,
        ))
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..num_points).map(|i| (i, speed[i])),
            &BLUE,
        ))
        .unwrap();
}
