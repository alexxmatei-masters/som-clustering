use chrono::{DateTime, Local};
use std::{
    fs::{self, File},
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
            let color = plotters::style::colors::full_palette::PINK_300;
            chart.draw_series(std::iter::once(Circle::new(
                (neuron.weights[0], neuron.weights[1]),
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

fn draw_randomly_selected_point(
    point: &(f64, f64),
    root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let color = plotters::style::colors::full_palette::PURPLE;
    chart.draw_series(std::iter::once(Circle::new(
        (point.0, point.1),
        5,
        color.filled(),
    )))?;
    Ok(())
}

fn draw_winner_neuron(
    point: &(f64, f64),
    root: &mut DrawingArea<BitMapBackend, plotters::coord::Shift>,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let color = plotters::style::colors::full_palette::RED;
    chart.draw_series(std::iter::once(Circle::new(
        (point.0, point.1),
        5,
        color.filled(),
    )))?;
    Ok(())
}

fn euclidean_distance(p1: &(f64, f64), p2: &(f64, f64)) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}

fn find_closest_neuron(point: &(f64, f64), network: &SOMNetwork) -> (usize, usize) {
    let mut closest_neuron_pos = (0, 0);
    let mut closest_distance = euclidean_distance(
        &(point.0, point.1),
        &(
            network.neurons[0][0].weights[0],
            network.neurons[0][0].weights[1],
        ),
    );
    for (row_index, row) in network.neurons.iter().enumerate() {
        for (col_index, neuron) in row.iter().enumerate() {
            let dist =
                euclidean_distance(&(point.0, point.1), &(neuron.weights[0], neuron.weights[1]));
            if dist < closest_distance {
                closest_neuron_pos = (row_index, col_index);
                closest_distance = dist;
            }
        }
    }

    closest_neuron_pos
}

fn main() {
    let local: DateTime<Local> = Local::now();
    let mut epoch = 0;
    let timestamp = local.format("%Y%m%d-%H%M%S").to_string();
    let folder_name = format!("plots_{}", timestamp);
    fs::create_dir(&folder_name).unwrap();
    let path1 = format!("{}/e{}_plot1.png", folder_name, epoch);

    let points = read_points_from_file();

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

    let mut som_network = SOMNetwork::new(10, 10, 2);
    let mut plot1 = BitMapBackend::new(&path1, (600, 600)).into_drawing_area();
    draw_points(&points, &mut plot1);
    draw_lines(&som_network, &mut plot1);
    draw_neurons(&som_network, &mut plot1);

    loop {
        const PROPOSED_ITERATION_NR: u8 = 10;
        let mut neighbourhood = 6.1
            * f64::powf(
                std::f64::consts::E,
                -epoch as f64 / PROPOSED_ITERATION_NR as f64,
            );
        let mut learning_rate = 0.4
            * f64::powf(
                std::f64::consts::E,
                -epoch as f64 / PROPOSED_ITERATION_NR as f64,
            );
        println!();
        println!("Epoch {}", epoch);
        println!("Learning rate: {:.5}", learning_rate);
        println!("Neighbourhood value: {:.3}", neighbourhood);

        const LEARNING_RATE_THRESHOLD: f32 = 0.001;
        if learning_rate <= LEARNING_RATE_THRESHOLD as f64 {
            println!(
                "Learning rate under {}, exiting program...",
                LEARNING_RATE_THRESHOLD
            );
            break;
        }

        for (index, point) in points.iter().enumerate() {
            let path2 = format!("{}/e{}_plot2_i{}.png", folder_name, epoch, index + 1);

            let winner_neuron_pos = find_closest_neuron(&point, &som_network);

            // Update weights for winner & neighbourhood
            for (i, row) in som_network.neurons.iter_mut().enumerate() {
                for (j, neuron) in row.iter_mut().enumerate() {
                    // if neuron belongs to the neighbourhood
                    if (i as i8 >= winner_neuron_pos.0 as i8 - neighbourhood as i8)
                        && (i as i8 <= winner_neuron_pos.0 as i8 + neighbourhood as i8)
                    {
                        if (j as i8 >= winner_neuron_pos.1 as i8 - neighbourhood as i8)
                            && (j as i8 <= winner_neuron_pos.1 as i8 + neighbourhood as i8)
                        {
                            neuron.weights[0] = neuron.weights[0] as f64
                                + learning_rate as f64 * (point.0 - neuron.weights[0]);
                            neuron.weights[1] = neuron.weights[1] as f64
                                + learning_rate as f64 * (point.1 - neuron.weights[1]);
                        }
                    }
                }
            }

            if (index == 9999) {
                let mut plot2 = BitMapBackend::new(&path2, (600, 600)).into_drawing_area();
                draw_points(&points, &mut plot2);
                draw_lines(&som_network, &mut plot2);
                draw_neurons(&som_network, &mut plot2);
                draw_randomly_selected_point(&point, &mut plot2);
                draw_winner_neuron(
                    &(
                        som_network.neurons[winner_neuron_pos.0][winner_neuron_pos.1].weights[0]
                            as f64,
                        som_network.neurons[winner_neuron_pos.0][winner_neuron_pos.1].weights[1]
                            as f64,
                    ),
                    &mut plot2,
                );
            }
        }
        epoch += 1;
    }
}
