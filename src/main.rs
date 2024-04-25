use colored::Colorize;
use log::{debug, error, info, LevelFilter, trace, warn};
use ndarray::{Array2, ArrayBase, Ix2, OwnedRepr};
use petgraph::algo::astar;
use petgraph::prelude::{EdgeRef, NodeIndex, StableGraph, StableUnGraph};
use petgraph::Undirected;
use petgraph::visit::IntoEdgeReferences;

use rand::distributions::{Distribution, Uniform};
use rand::{Rng, thread_rng};

const NODES: usize = 20;

fn main() {
    // env_logger::init();
    env_logger::builder().filter_level(LevelFilter::Debug).init();
    // let mut intensity = [[0usize; NODES]; NODES];

    info!("Constructing graph");
    let mut graph = StableUnGraph::<u8, Connection>::with_capacity(NODES, 30);
    trace!("Graph created");

    let mut nodes = Vec::new();
    for _ in 0..NODES {
        nodes.push(graph.add_node(0));
    }
    trace!("Nodes added");

    set_edges(&mut graph, &nodes);

    let mut intensity = Array2::<usize>::zeros((NODES, NODES));
    set_intensity(&mut intensity);

    set_flow(&mut graph, &nodes, &intensity);
    test_network(&graph, &intensity);

    // monte_carlo();
    // experiment1();
    // experiment2();
    // experiment3();


    // random_disconnect(&mut graph);
    // reset_flow(&mut graph);
    // set_flow(&mut graph, &nodes, &intensity);
    // test_network(&graph, &intensity);
}


fn init_graph() -> (StableGraph<u8, Connection, Undirected>, Vec<NodeIndex>, ArrayBase<OwnedRepr<usize>, Ix2>) {
    let _ = env_logger::builder().filter_level(LevelFilter::Debug).is_test(true).try_init();
    // let mut intensity = [[0usize; NODES]; NODES];

    info!("Constructing graph");
    let mut graph = StableUnGraph::<u8, Connection>::with_capacity(NODES, 30);
    trace!("Graph created");

    let mut nodes = Vec::new();
    for _ in 0..NODES {
        nodes.push(graph.add_node(0));
    }
    trace!("Nodes added");

    set_edges(&mut graph, &nodes);

    let mut intensity = Array2::<usize>::zeros((NODES, NODES));
    set_intensity(&mut intensity);
    (graph, nodes, intensity)
}


fn random_disconnect(graph: &mut StableGraph<u8, Connection, Undirected>) {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0., 1.);

    for i in graph.edge_indices().collect::<Vec<_>>() {
        let chance = graph.edge_weight(i).unwrap().stability;
        let rand = range.sample(&mut rng);
        if rand > chance {
            graph.remove_edge(i);
            trace!("{} {:?}", "Connection failed", i);
        }
    }
    trace!("Checked for failing connections");
}

fn test_network(graph: &StableUnGraph<u8, Connection>, intensity: &Array2<usize>) -> f64 {
    let mut sum_e = 0.;
    let mut over = 0;
    for e in graph.edge_references() {
        let weight = e.weight();
        let (a, c) = (weight.flow as f64, weight.bandwidth as f64);
        let mut val = 1.;
        let _msg = if a <= c {
            val = c - a;
            "Flow Good".green()
        } else {
            over += 1;
            // warn!("Flow too high on connection {}-{}", e.source().index(), e.target().index());
            "Flow too high".red()
        };

        sum_e += a / val;
        // trace!("Connection {}-{} Flow: {}, {}", e.source().index(), e.target().index(), weight.flow, msg)
    }
    if over > 0 {
        warn!("{} Connections overloaded", over);
    }
    let t = 1. / intensity.sum() as f64 * sum_e;
    debug!("Value T: {}", t);
    t
}

fn set_edges(graph: &mut StableUnGraph<u8, Connection>, nodes: &Vec<NodeIndex>) {
    graph.add_edge(nodes[0], nodes[3], Connection::new(1000));
    graph.add_edge(nodes[1], nodes[3], Connection::new(1000));
    graph.add_edge(nodes[2], nodes[3], Connection::new(1000));
    graph.add_edge(nodes[3], nodes[4], Connection::new(10_000));
    graph.add_edge(nodes[4], nodes[5], Connection::new(10_000));
    graph.add_edge(nodes[5], nodes[6], Connection::new(1000));
    graph.add_edge(nodes[5], nodes[7], Connection::new(1000));
    graph.add_edge(nodes[5], nodes[8], Connection::new(1000));
    graph.add_edge(nodes[4], nodes[9], Connection::new(100_000));
    graph.add_edge(nodes[9], nodes[10], Connection::new(10_000));
    graph.add_edge(nodes[10], nodes[11], Connection::new(1000));
    graph.add_edge(nodes[10], nodes[12], Connection::new(1000));
    graph.add_edge(nodes[9], nodes[13], Connection::new(100_000));
    graph.add_edge(nodes[13], nodes[14], Connection::new(10_000));
    graph.add_edge(nodes[14], nodes[15], Connection::new(1000));
    graph.add_edge(nodes[14], nodes[16], Connection::new(1000));
    graph.add_edge(nodes[13], nodes[17], Connection::new(10_000));
    graph.add_edge(nodes[17], nodes[18], Connection::new(1000));
    graph.add_edge(nodes[17], nodes[19], Connection::new(1000));

    graph.add_edge(nodes[3], nodes[5], Connection::new(10_000));
    graph.add_edge(nodes[3], nodes[10], Connection::new(10_000));
    graph.add_edge(nodes[5], nodes[14], Connection::new(10_000));
    graph.add_edge(nodes[14], nodes[17], Connection::new(10_000));
    graph.add_edge(nodes[10], nodes[17], Connection::new(10_000));

    trace!("Edges added");
}

fn set_edges2(graph: &mut StableUnGraph<u8, Connection>, nodes: &Vec<NodeIndex>) {
    let mut rng = rand::thread_rng();
    let node_range = Uniform::new(0, NODES);
    let bandwidth_range = Uniform::new(1000, 10000);
    for i in 0..NODES {
        graph.add_edge(nodes[i], nodes[node_range.sample(&mut rng)], Connection::new(bandwidth_range.sample(&mut rng)));
    }
    for _i in 0..10 {
        graph.add_edge(nodes[node_range.sample(&mut rng)], nodes[node_range.sample(&mut rng)], Connection::new(bandwidth_range.sample(&mut rng)));
    }
    trace!("Edges added");
}

fn set_intensity(intensity: &mut Array2<usize>) {
    let small = vec![0, 1, 2, 6, 7, 8, 11, 12, 15, 16, 18, 19];
    let medium = vec![3, 5, 10, 14, 17];
    let big = vec![4, 9, 13];


    let mut set_intensity = |index1: usize, index2: usize, value: usize| {
        if index1 < index2 {
            intensity[[index1, index2]] = value;
        } else {
            intensity[[index2, index1]] = value;
        }
    };

    for i in 0..small.len() {
        for j in (i + 1)..small.len() {
            set_intensity(small[i], small[j], 10);
        }
    }

    for i in &small {
        for j in &medium {
            set_intensity(*i, *j, 20);
        }
    }

    for i in &small {
        for j in &big {
            set_intensity(*i, *j, 30);
        }
    }

    for i in 0..medium.len() {
        for j in (i + 1)..medium.len() {
            set_intensity(medium[i], medium[j], 40);
        }
    }

    for i in &medium {
        for j in &big {
            set_intensity(*i, *j, 60);
        }
    }

    for i in 0..big.len() {
        for j in (i + 1)..big.len() {
            set_intensity(big[i], big[j], 80);
        }
    }

    trace!("Intensity set");
}

fn reset_flow(graph: &mut StableUnGraph<u8, Connection>) {
    for e in graph.edge_weights_mut() {
        e.flow = 0;
    }
}

fn set_flow(graph: &mut StableUnGraph<u8, Connection>, nodes: &Vec<NodeIndex>, intensity: &Array2<usize>) -> bool {
    let mut disconnected = 0;
    for i in 0..NODES {
        for j in (i + 1)..NODES {
            let val = intensity[[i, j]];
            let path = astar(&*graph, nodes[i], |finish| finish == nodes[j], |_| 0, |_| 0);
            if let Some((_, path)) = path {
                for w in path.windows(2) {
                    let edge = graph.find_edge(w[0], w[1]).unwrap();
                    let connection = graph.edge_weight_mut(edge).unwrap();
                    connection.flow += val;
                }
            } else {
                disconnected += 1;
            }
        }
    }
    trace!("Flow set, Disconnected pairs of nodes: {}", disconnected);
    return if disconnected > 0 {
        warn!("{}", "Network disjointed!");
        false
    } else {
        true
    };
}

struct Connection {
    bandwidth: usize,
    flow: usize,
    stability: f64,
}

impl Connection {
    fn new(bandwidth: usize) -> Connection {
        Connection {
            bandwidth,
            flow: 0,
            stability: thread_rng().gen_range(0.9..1.),
        }
    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        Connection {
            bandwidth: self.bandwidth,
            flow: self.flow,
            stability: self.stability,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monte_carlo() {
        let (mut graph,mut nodes,mut intensity) = init_graph();

        info!("{}", "Experiment monte carlo".yellow());
        let mut graph = graph.clone();
        let mut data = vec![];
        let mut failed = 0;
        let iterations = 1000;
        for i in 0..iterations {
            debug!("Iteration {}", i);
            let mut graph = graph.clone();
            random_disconnect(&mut graph);
            reset_flow(&mut graph);
            let res = set_flow(&mut graph, &nodes, &intensity);
            let t = test_network(&graph, &intensity);
            if res {
                data.push(t);
            } else {
                failed += 1;
            }
        }
        info!("Finished");
        info!("Average T value: {}", data.iter().sum::<f64>()/data.len() as f64);
        info!("Failed attempts: {} / {}", failed, iterations)
    }

    #[test]
    fn experiment1() {
        let (mut graph,mut nodes,mut intensity) = init_graph();

        info!("{}", "Experiment 1".yellow());
        let mut intensity = intensity.clone();
        let mut graph = graph.clone();
        for i in 0..100 {
            debug!("Iteration {}", i);
            intensity.mapv_inplace(|x| (x as f64 * 1.05) as usize);

            // let mut graph = graph.clone();
            // random_disconnect(&mut graph);
            reset_flow(&mut graph);
            set_flow(&mut graph, &nodes, &intensity);
            test_network(&graph, &intensity);
        }
        info!("Finished");
        println!("{}", "NICCCC".red())
    }


    #[test]
    fn experiment2() {
        let (mut graph,mut nodes,mut intensity) = init_graph();

        info!("{}", "Experiment 2".yellow());
        let mut graph = graph.clone();
        for i in 0..100 {
            debug!("Iteration {}", i);
            graph.edge_weights_mut().for_each(|x| x.bandwidth = (x.bandwidth as f64 * 1.05) as usize);

            // let mut graph = graph.clone();
            // random_disconnect(&mut graph);
            reset_flow(&mut graph);
            set_flow(&mut graph, &nodes, &intensity);
            test_network(&graph, &intensity);
        }
        info!("Finished");
    }

    #[test]
    fn experiment3() {
        let (mut graph,mut nodes,mut intensity) = init_graph();

        info!("{}", "Experiment 3".yellow());
        let mut graph = graph.clone();
        let average = graph.edge_references().fold(0., |acc, x| acc + x.weight().bandwidth as f64) / graph.edge_count() as f64;
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, NODES);
        for i in 0..20 {
            debug!("Iteration {}", i);
            graph.add_edge(nodes[range.sample(&mut rng)], nodes[range.sample(&mut rng)], Connection::new(average as usize));


            // let mut graph = graph.clone();
            // random_disconnect(&mut graph);
            reset_flow(&mut graph);
            set_flow(&mut graph, &nodes, &intensity);
            test_network(&graph, &intensity);
        }
        info!("Finished");
    }

}