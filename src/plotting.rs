use std::error::Error;

use gpx::{Gpx, Track, TrackSegment};
use plotters::prelude::*;

pub fn create_plot(gpx: &Gpx, output_file: &str) -> Result<(), Box<dyn Error>> {
    // Each GPX file has multiple "tracks", this takes the first one.
    let track: &Track = &gpx.tracks[0];

    // Each track will have different segments full of waypoints, where a
    // waypoint contains info like latitude, longitude, and elevation.
    let segment: &TrackSegment = &track.segments[0];
    let num_points: usize = segment.points.len();
    let elevation: Vec<f64> = (0..num_points)
        .map(|i| segment.points[i].elevation.unwrap())
        .collect();
    let elevation = lowpass_filter(&elevation, 75);

    let drawing_area = BitMapBackend::new(output_file, (1024, 768)).into_drawing_area();

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

    Ok(())
}

fn lowpass_filter(data: &Vec<f64>, order: usize) -> Vec<f64> {
    let mut res = Vec::<f64>::new();
    let mut accumulator: f64 = 0.0;
    let mut elems_count: usize = 0;

    for i in 0..data.len() {
        if i >= order {
            accumulator -= data[i - order];
            elems_count -= 1;
        }

        elems_count += 1;
        accumulator += data[i];

        res.push(accumulator / elems_count as f64);
    }

    res
}
