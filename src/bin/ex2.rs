use colored::Colorize;
use log::{info, LevelFilter, warn};
use sieci::{init_graph, reliability};

fn main() {
    let _ = env_logger::builder().filter_level(LevelFilter::Info).is_test(true).try_init();
    let (mut graph, nodes, intensity) = init_graph();

    warn!("{}", "Experiment 2".yellow());
    for i in 0..100 {
        warn!("Iteration {}", i);
        graph.edge_weights_mut().for_each(|x| x.bandwidth = (x.bandwidth as f64 * 1.01) as usize);

        reliability(&graph, &nodes, &intensity,  0.0000000012);
    }
    warn!("Finished");
}