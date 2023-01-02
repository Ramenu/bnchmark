use std::env;
use std::error::Error;
use std::process::Command;
use std::time::Instant;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let cmd1 = &args[1];
    let cmd1_args: Vec<&str> = cmd1.split(" ").collect();
    let cmd2 = &args[2];
    let cmd2_args: Vec<&str> = cmd2.split(" ").collect();

    let n = args[3].parse::<usize>().expect("Not a valid number");

    let mut cmd1_times = Vec::new();
    let mut cmd2_times = Vec::new();

    for _ in 0..n {
        // Run command 1 and record execution time
        let start = Instant::now();
        Command::new(&cmd1_args[0]).args(&cmd1_args[1..]).output().expect("Failed to execute command 1");
        let end = start.elapsed();
        cmd1_times.push(end.as_secs_f64());

        // Run command 2 and record execution time
        let start = Instant::now();
        Command::new(&cmd2_args[0]).args(&cmd2_args[1..]).output().expect("Failed to execute command 2");
        let end = start.elapsed();
        cmd2_times.push(end.as_secs_f64());
    }

    // Calculate mean execution time for each command
    let cmd1_mean = cmd1_times.iter().sum::<f64>() / (n as f64);
    let cmd2_mean = cmd2_times.iter().sum::<f64>() / (n as f64);

    let cmd1_variance = cmd1_times.iter().map(|x| (x - cmd1_mean).powi(2)).sum::<f64>() / (n as f64);
    let cmd1_stddev = cmd1_variance.sqrt();

    let cmd2_variance = cmd2_times.iter().map(|x| (x - cmd2_mean).powi(2)).sum::<f64>() / (n as f64);
    let cmd2_stddev = cmd2_variance.sqrt();

    println!("Executed benchmark {} times:\n\
              '{}' mean execution time: {:.3} seconds\n\
              '{}' standard deviation: {:.3}\n\n\
              '{}' mean execution time: {:.3} seconds\n\
              '{}' standard deviation: {:.3}", n, &cmd1, cmd1_mean, &cmd1, cmd1_stddev, &cmd2, cmd2_mean, &cmd2, cmd2_stddev);
}