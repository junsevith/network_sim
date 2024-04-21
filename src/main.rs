use petgraph::graph::UnGraph;

fn main() {
    const NODES: usize = 20;
    let mut intensity = [[0; NODES]; NODES];

    let mut graph = UnGraph::new_undirected();
    let mut nodes = Vec::new();
    for _ in 0..NODES {
        nodes.push(graph.add_node(0));
    }

    graph.add_edge(nodes[0], nodes[3], 1000);
    graph.add_edge(nodes[1], nodes[3], 1000);
    graph.add_edge(nodes[2], nodes[3], 1000);
    graph.add_edge(nodes[3], nodes[4], 10_000);
    graph.add_edge(nodes[4], nodes[5], 10_000);
    graph.add_edge(nodes[5], nodes[6], 1000);
    graph.add_edge(nodes[5], nodes[7], 1000);
    graph.add_edge(nodes[5], nodes[8], 1000);
    graph.add_edge(nodes[4], nodes[9], 100_000);
    graph.add_edge(nodes[9], nodes[10], 10_000);
    graph.add_edge(nodes[10], nodes[11], 1000);
    graph.add_edge(nodes[10], nodes[12], 1000);
    graph.add_edge(nodes[9], nodes[13], 100_000);
    graph.add_edge(nodes[13], nodes[14], 10_000);
    graph.add_edge(nodes[14], nodes[15], 1000);
    graph.add_edge(nodes[14], nodes[16], 1000);
    graph.add_edge(nodes[13], nodes[17], 10_000);
    graph.add_edge(nodes[17], nodes[18], 1000);
    graph.add_edge(nodes[17], nodes[19], 1000);

    let small = vec![0, 1, 2, 6, 7, 8, 11, 12, 15, 16, 18, 19];
    let medium = vec![3, 5, 10, 14, 17];
    let big = vec![4, 9, 13];


    {
        let mut set_intensity = |index1: usize, index2: usize, value: i32| {
            if index1 < index2 {
                intensity[index1][index2] = value;
            } else {
                intensity[index2][index1] = value;
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
    }

    let mut real_intensity = [[0; NODES]; NODES];

    {
        let mut inc_real_intensity = |index1: usize, index2: usize, value: i32| {
            if index1 < index2 {
                real_intensity[index1][index2] += value;
            } else {
                real_intensity[index2][index1] += value;
            }
        };

        for i in 0..NODES {
            for j in (i + 1)..NODES {
                let val = intensity[i][j];
                let path = petgraph::algo::astar(&graph, nodes[i], |finish| finish == nodes[j], |_| 0, |_| 0);
                if let Some((_, path)) = path {
                    let mut first = 0;
                    let mut second = 1;
                    while second < path.len() {
                        inc_real_intensity(path[first].index(), path[second].index(), val);
                        first += 1;
                        second += 1;
                    }
                }
            }
        }
    }

    let display_matrix = |matrix: &[[i32; NODES];NODES]| {
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let val = matrix[i][j];
                if val != 0 {
                    println!("{} {} {}", i, j, val)
                }
            }
        }
    };

    display_matrix(&real_intensity);
}



