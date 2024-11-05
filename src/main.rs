use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use nalgebra::Matrix3;
use ndarray::arr2;
use crate::dijkstra_find_path::DijkstraSP;
use crate::floyd_find_path::FloydSP;
use crate::graph::EdgeWeightedDigraph;

mod bag;
mod dijkstra_find_path;
mod index_min_pq;
mod graph;
mod frank_wolf;
mod floyd_find_path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let start = Instant::now();
    let data = read_lines("data/test_small.txt");
    if data.is_err() {
        return;
    }
    let lines =  data.ok().expect("Error").flatten();
    let mut lines_data = &mut vec![];
    for line in lines {
        lines_data.push(line.trim().to_string())
    }


    //запуск алгоритма флойда
    let mut g = EdgeWeightedDigraph::default_graph();
    let mut graph = EdgeWeightedDigraph::graph_from_array_str_with_matrix(& mut g, lines_data.to_vec());

    let fl = FloydSP::floyd(graph);

    let u = 0usize;
    let v = 1usize;
    let d = fl.path_to(u, v).unwrap();
    println!("растояние -  {}", fl.dist_to(u, v));
    for t in d {
        print!("{} ", t);
    }

    println!();

    // запуск алгоритма Дейкстры
    let mut g = EdgeWeightedDigraph::default_graph();
    let mut graph_adj = EdgeWeightedDigraph::graph_from_array_str(& mut g, lines_data.to_vec());

    let s = 0;

    let sp = DijkstraSP::dijkstra(graph_adj, s);

    let duration = start.elapsed();

    for t in 0..graph.get_v_count() {
        print!("{} to {} ", s, t);
        print!("({})", sp.dist_to(t as usize));
        if sp.has_path_to(t as usize) {
            for e in sp.path_to(t as usize).expect("Error").iter() {
                print!(" ");
                e.to_string();
                print!(" ");
            }
        }
        println!()
    }
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
