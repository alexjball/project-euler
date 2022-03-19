use std::{collections::HashSet, fmt::Display, fs, time::Instant};

pub fn run() {
    let example = "data/p107_example.txt";
    let actual = "data/p107_network.txt";
    let adj = AdjacencyList::from_file(actual);

    let now = Instant::now();
    let savings = Prim::compute(adj);
    println!("{:?} {}", now.elapsed(), savings);
}

struct Prim {
    graph: AdjacencyList,
    mst: AdjacencyList,
    mst_sum: i32,
}

impl Prim {
    fn compute(graph: AdjacencyList) -> i32 {
        let mut prim = Prim {
            mst: AdjacencyList::new(graph.n_vertices),
            graph,
            mst_sum: 0,
        };

        prim.run();

        let total_weight = prim.graph.weights.iter().flatten().sum::<i32>() / 2;
        return total_weight - prim.mst_sum;
    }

    /// 1. Select an arbitrary vertex as the start of the MST and set T = v
    /// 2. Select the lowest-weight edge connecting T to a new vertex and add it to the mst
    /// 3. Repeat 2 until all vertices are included
    fn run(&mut self) {
        let mut mst_v = HashSet::from([0]);
        let mut sum = 0;
        loop {
            let smallest_edge = mst_v
                .iter()
                .map(|&v| {
                    self.graph
                        .get_edges(v)
                        .unwrap()
                        .iter()
                        .enumerate()
                        .filter(|&(v, &w)| w > 0 && !mst_v.contains(&(v as i32)))
                        .min_by_key(|x| x.1)
                })
                .flatten()
                .min_by_key(|x| x.1);
            match smallest_edge {
                Some((v, w)) => {
                    sum += w;
                    mst_v.insert(v as i32);
                }
                None => {
                    self.mst_sum = sum;
                    break;
                }
            }
        }
    }
}

#[derive(Debug)]
struct AdjacencyList {
    weights: Vec<Vec<i32>>,
    n_vertices: i32,
}

impl AdjacencyList {
    fn new(n_vertices: i32) -> AdjacencyList {
        AdjacencyList {
            n_vertices,
            weights: vec![vec![0; n_vertices as usize]; n_vertices as usize],
        }
    }

    fn from_file(file: &str) -> AdjacencyList {
        let weights = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|cell| cell.parse::<i32>().or::<()>(Ok(0)).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let n_vertices = weights.len();

        if weights.iter().any(|row| row.len() != n_vertices) {
            panic!(
                "Invalid weights, expected {} rows with {} elements",
                n_vertices, n_vertices
            );
        }

        return AdjacencyList {
            weights,
            n_vertices: n_vertices as i32,
        };
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&i32> {
        self.weights.get(row).and_then(|row| row.get(col))
    }

    pub fn get_edges(&self, v: i32) -> Option<&Vec<i32>> {
        self.weights.get(v as usize)
    }
}

impl Display for AdjacencyList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} vertices [\n", self.n_vertices)?;
        for row in &self.weights {
            write!(f, "  [")?;
            row.iter().for_each(|c| write!(f, "{:<3} ", c).unwrap());
            writeln!(f, "]")?;
        }
        write!(f, "\n]")
    }
}
