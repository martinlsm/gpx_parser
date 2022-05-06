use std::error::Error;

use gpx::{Gpx, Track, TrackSegment};
use plotters::prelude::*;

use crate::error::GpxError;

// TODO: Remove all the unwraps.
pub fn create_plot(gpx: &mut Gpx, output_file: &str) -> Result<(), Box<dyn Error>> {
    if gpx.tracks.len() != 1 {
        return Err(Box::new(GpxError::new(
            "GPX file must have exactly one track contained in it",
        )));
    }
    let track: &Track = &gpx.tracks[0];

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

    let height: u32 = 768;
    let width: u32 = 1024;
    let root = BitMapBackend::new(output_file, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically(height / 2);

    // Plot elevation.

    let plot_max = elevation
        .iter()
        .fold(f64::NEG_INFINITY, |x, &y| if x > y { x } else { y });

    let mut chart = ChartBuilder::on(&upper)
        .caption("Elevation", ("sans-serif", (10).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (15).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (10).percent())
        .margin((1).percent())
        .build_cartesian_2d(0..num_points, -10.0..plot_max + 10.0)?;

    let color = Palette99::pick(4).mix(0.9);

    chart
        .draw_series(LineSeries::new(
            (0..num_points).map(|i| (i, elevation[i])),
            color.stroke_width(2),
        ))
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Time (TODO)") // TODO: Fix plotting of time.
        .y_desc("Elevation (m)")
        .draw()?;

    // Plot speed.

    let plot_max = speed
        .iter()
        .fold(f64::NEG_INFINITY, |x, &y| if x > y { x } else { y });

    let mut chart = ChartBuilder::on(&lower)
        .caption("Speed", ("sans-serif", (10).percent_height()))
        .set_label_area_size(LabelAreaPosition::Left, (15).percent())
        .set_label_area_size(LabelAreaPosition::Bottom, (10).percent())
        .margin((1).percent())
        .build_cartesian_2d(0..num_points, 1.0..plot_max + 4.0)?;

    let color = Palette99::pick(3).mix(0.9);

    chart
        .configure_mesh()
        .x_desc("Time (TODO)") // TODO: Fix plotting of time.
        .y_desc("Speed (m/s)")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..num_points).map(|i| (i, speed[i])),
            color.stroke_width(2),
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
