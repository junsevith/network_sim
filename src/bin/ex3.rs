use colored::Colorize;
use log::{debug, info, LevelFilter, warn};
use petgraph::visit::IntoEdgeReferences;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::thread_rng;
use sieci::{Connection, init_graph, reliability};


fn main() {
    let _ = env_logger::builder().filter_level(LevelFilter::Debug).is_test(true).try_init();
    let (graph, nodes, intensity) = init_graph();

    warn!("Experiment 3");
    let mut graph = graph.clone();
    let average = graph.edge_references().fold(0., |acc, x| acc + x.weight().bandwidth as f64) / graph.edge_count() as f64;
    let mut rng = thread_rng();
    let range = Uniform::new(0, sieci::NODES);
    for i in 0..20 {
        warn!("Iteration {}", i);
        graph.add_edge(nodes[range.sample(&mut rng)], nodes[range.sample(&mut rng)], Connection::new(average as usize));

        reliability(&graph, &nodes, &intensity, 0.0000000012);
    }
    warn!("Finished");
}