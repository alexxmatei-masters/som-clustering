use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use plotters::prelude::*;
use plotters::style::Color;
struct SOMNeuron {
    weights: Vec<f64>,
}

struct SOMNetwork {
    neurons: Vec<Vec<SOMNeuron>>,
}
fn read_points_from_file() -> Vec<(f64, f64)> {
    let file = File::open("points.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut points = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        let x = parts[0].parse::<f64>().unwrap();
        let y = parts[1].parse::<f64>().unwrap();
        points.push((x, y));
    }
    return points;
}

fn draw_lines(
    network: &SOMNetwork,
    root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>,
) -> Result<(), Box<dyn std::error::Error>> {
    let x_min = -300.0;
    let x_max = 300.0;
    let y_min = -300.0;
    let y_max = 300.0;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption("SOM Network", ("sans-serif", 50.0))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().axis_style(&BLACK).draw()?;

    for (i, row) in network.neurons.iter().enumerate() {
        for (j, neuron) in row.iter().enumerate() {
            if i > 0 {
                // Draw a line between this neuron and the neuron above it
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![
                        (neuron.weights[0], neuron.weights[1]),
                        (
                            network.neurons[i - 1][j].weights[0],
                            network.neurons[i - 1][j].weights[1],
                        ),
                    ],
                    plotters::style::colors::BLACK.filled().stroke_width(1),
                )))?;
            }
            if j > 0 {
                // Draw a line between this neuron and the neuron to its left
                let color = plotters::style::colors::full_palette::BLACK;
                chart.draw_series(std::iter::once(PathElement::new(
                    vec![
                        (neuron.weights[0], neuron.weights[1]),
                        (
                            network.neurons[i][j - 1].weights[0],
                            network.neurons[i][j - 1].weights[1],
                        ),
                    ],
                    color.filled().stroke_width(1),
                )))?;
            }
        }
    }
    Ok(())
}

fn draw_neurons(
    network: &SOMNetwork,
    root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>,
) -> Result<(), Box<dyn std::error::Error>> {
    let x_min = -300.0;
    let x_max = 300.0;
    let y_min = -300.0;
    let y_max = 300.0;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption("SOM Network", ("sans-serif", 50.0))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().axis_style(&BLACK).draw()?;

    for (i, row) in network.neurons.iter().enumerate() {
        for (j, neuron) in row.iter().enumerate() {
            let color = plotters::style::colors::full_palette::BLUE;
            chart.draw_series(std::iter::once(Circle::new(
                (
                    -300.0 + 30.0 + (i as f64 * 60.0),
                    -300.0 + 30.0 + (j as f64 * 60.0),
                ),
                3,
                color.filled(),
            )))?;
        }
    }
    Ok(())
}

fn draw_points(
    points: &Vec<(f64, f64)>,
    root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize plot to be empty
    root.fill(&WHITE)?;

    // Define the dimensions and layout of the plot
    let x_min = -300.0;
    let x_max = 300.0;
    let y_min = -300.0;
    let y_max = 300.0;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption("SOM Network", ("sans-serif", 50.0))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().axis_style(&BLACK).draw()?;

    // Add the points to the chart
    for point in points {
        let color = plotters::style::colors::full_palette::GREY;
        chart.draw_series(std::iter::once(Circle::new(
            (point.0, point.1),
            2,
            color.filled(),
        )))?;
    }
    Ok(())
}

fn main() {
    let points = read_points_from_file();
    println!("{:?}", points);

    impl SOMNetwork {
        fn new(num_rows: usize, num_cols: usize, input_dim: usize) -> Self {
            let mut neurons = Vec::with_capacity(num_rows);
            for row_nr in 0..num_rows {
                let mut row = Vec::with_capacity(num_cols);
                for col_nr in 0..num_cols {
                    let mut weights = Vec::with_capacity(input_dim);
                    for pos in 0..input_dim {
                        let mut neuron_position: f64 = 0.0;
                        if pos == 0 {
                            neuron_position = -300.0 + 30.0 + (row_nr as f64 * 60.0);
                        }
                        if pos == 1 {
                            neuron_position = -300.0 + 30.0 + (col_nr as f64 * 60.0);
                        }
                        weights.push(neuron_position);
                    }
                    row.push(SOMNeuron { weights });
                }
                neurons.push(row);
            }
            SOMNetwork { neurons }
        }
    }
    impl std::fmt::Debug for SOMNeuron {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SOMNeuron")
                .field("weights", &self.weights)
                .finish()
        }
    }

    impl std::fmt::Debug for SOMNetwork {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SOMNetwork")
                .field("neurons", &self.neurons)
                .finish()
        }
    }

    let som_network = SOMNetwork::new(10, 10, 2);
    for (index, row) in som_network.neurons.iter().enumerate() {
        println!("\nRow #{}:", index);
        for neuron in row {
            println!("{:?}", neuron)
        }
    }

    let path1 = "./plot1.png";
    let mut plot1 = BitMapBackend::new(&path1, (600, 600)).into_drawing_area();
    draw_points(&points, &mut plot1);
    draw_neurons(&som_network, &mut plot1);
    draw_lines(&som_network, &mut plot1);
}
