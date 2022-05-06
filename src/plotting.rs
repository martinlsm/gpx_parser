use std::error::Error;
use std::f64::consts;

use gpx::{Gpx, Track, TrackSegment};
use plotters::prelude::*;

const EARTH_RADIUS_METERS: f64 = 6371.0 * 1000.0;
const EARTH_CIRCUMFERENCE: f64 = EARTH_RADIUS_METERS * 2.0 * consts::PI;

pub fn create_plot(gpx: &mut Gpx, output_file: &str) -> Result<(), Box<dyn Error>> {
    println!("GPX Version: {}", gpx.version);

    fill_speed_2(gpx);

    // TODO: Each GPX file has multiple "tracks", this takes the first one.
    let track: &Track = &gpx.tracks[0];

    // Each track will have different segments full of waypoints, where a
    // waypoint contains info like latitude, longitude, and elevation.
    let segment: &TrackSegment = &track.segments[0];
    let num_points: usize = segment.points.len();

    let elevation: Vec<f64> = (0..num_points)
        .map(|i| segment.points[i].elevation.unwrap())
        .collect();
    let elevation = lowpass_filter(&elevation, 75);

    let speed: Vec<f64> = (0..num_points)
        .map(|i| segment.points[i].speed.unwrap())
        .collect();
    let speed = lowpass_filter(&speed, 75);

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

    chart
        .draw_series(LineSeries::new(
            (0..num_points).map(|i| (i, speed[i])),
            &BLUE,
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

fn fill_speed(gpx: &mut Gpx) {
    let track: &mut Track = &mut gpx.tracks[0];
    let segment: &mut TrackSegment = &mut track.segments[0];

    segment.points[0].speed = Some(0.0);
    for i in 1..segment.points.len() {
        // XXX: Extreme niche-cache: What happens if -180 wraps around to 180?
        let p0 = segment.points[i - 1].point();
        let p1 = segment.points[i].point();
        let diff = p1 - p0;
        let degrees_diff = diff.x().hypot(diff.y());
        let radians_diff = degrees_diff / 360.0 * 2.0 * consts::PI;

        let time_diff = segment.points[i].time.unwrap().timestamp() - segment.points[i - 1].time.unwrap().timestamp();

        let speed = EARTH_CIRCUMFERENCE * radians_diff / time_diff as f64;

        println!("{:?}", speed);
        segment.points[i].speed = Some(speed);
    }
}

fn fill_speed_2(gpx: &mut Gpx) {
    let track: &mut Track = &mut gpx.tracks[0];
    let segment: &mut TrackSegment = &mut track.segments[0];

    segment.points[0].speed = Some(0.0);
    for i in 1..segment.points.len() {
        let p0 = segment.points[i - 1].point();
        let p0 = to_x_y_z((p0.x(), p0.y()));
        let p1 = segment.points[i].point();
        let p1 = to_x_y_z((p1.x(), p1.y()));
        let speed = (p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2) * (p1.2 - p0.2).powi(2);

        println!("{:?}", speed);
        segment.points[i].speed = Some(speed);
    }
}

fn to_x_y_z(gpx_point: (f64, f64)) -> (f64, f64, f64) {
    let theta = gpx_point.0.to_radians();
    let phi = gpx_point.1.to_radians();
    let x = EARTH_RADIUS_METERS * theta.cos() * phi.sin();
    let y = EARTH_RADIUS_METERS * theta.sin() * phi.sin();
    let z = EARTH_RADIUS_METERS * phi.cos();

    (x, y, z)
}
