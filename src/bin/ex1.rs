use colored::Colorize;
use log::{info, LevelFilter, warn};
use sieci::{init_graph, reliability};


fn main() {
    env_logger::builder().filter_level(LevelFilter::Trace).is_test(true).try_init().unwrap();
    let (graph, nodes, mut intensity) = init_graph();

    warn!("{}", "Experiment 1".yellow());
    for i in 0..100 {
        warn!("Iteration {}", i);
        intensity.mapv_inplace(|x| (x as f64 * 1.01) as usize);

        reliability(&graph, &nodes, &intensity, 0.0000000014);
    }
    warn!("Finished");
}