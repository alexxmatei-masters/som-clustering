use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

fn main() {
    let points = read_points_from_file();
    println!("{:?}", points);

    struct SOMNeuron {
        weights: Vec<f64>,
    }

    struct SOMNetwork {
        neurons: Vec<Vec<SOMNeuron>>,
    }

    impl SOMNetwork {
        fn new(num_rows: usize, num_cols: usize, input_dim: usize) -> Self {
            let mut neurons = Vec::with_capacity(num_rows);
            for _ in 0..num_rows {
                let mut row = Vec::with_capacity(num_cols);
                for _ in 0..num_cols {
                    let mut weights = Vec::with_capacity(input_dim);
                    for _ in 0..input_dim {
                        // Initialize weight vector with random values between 0 and 1
                        weights.push(rand::random::<f64>());
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

    let som_network = SOMNetwork::new(5, 5, 2);
    for (index, row) in som_network.neurons.iter().enumerate() {
        println!("\nRow #{}:", index);
        for neuron in row {
            println!("{:?}", neuron)
        }
    }
}
