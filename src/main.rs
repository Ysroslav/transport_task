use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Deref;
use std::path::Path;
use std::time::Instant;

use crate::graph::EdgeWeightedDigraph;
use crate::parser_xml::parse_xml_to_structure;
use crate::projection_method::ProjectionMethod;

mod bag;
mod dijkstra_find_path;
mod index_min_pq;
mod graph;
mod frank_wolf;
mod floyd_find_path;
mod utils_graph;
mod delay_func_count;
mod structure_xml;
mod parser_xml;
mod projection_method;

fn main() {
    let start = Instant::now();

    let p = parse_xml_to_structure("C:\\Users\\Dell\\mipt\\abilene.xml");

    let mut point_index = HashMap::new();

    // индексируем вершины графа, для более быстрого расчета алгоритма Дейкстры
    let nodes = p.get_network_structure().get_nodes().get_node_vec();
    for (i, node) in nodes.iter().enumerate() {
        point_index.insert(node.get_id(), i as i32);
    }

    let commodities = p.get_demands().get_demand_vec();


    let N = 1000;

    let alpha: f64 = 0.99;
    //определяем граф
    let mut g = EdgeWeightedDigraph::default_graph();
    let graph_adj = EdgeWeightedDigraph::graph_from_struct_xml(&mut g, &p, &point_index);

    let mut result = HashMap::new();

    for (index, commodity) in commodities.iter().enumerate() {
        let source: &i32 = point_index.get(&commodity.get_source()).unwrap();
        let target: &i32 = point_index.get(&commodity.get_target()).unwrap();
        let r_k = commodity.get_demand_vale();
        let r_index = index as i32;
        let mut projection_handler = ProjectionMethod::new(N, alpha);
        
        let result_commodity = projection_handler.count_by_projection_method(graph_adj, *source, *target, r_k, r_index);
        result.insert(commodity.get_source() + "_" + &commodity.get_target(), result_commodity);
    }
    let duration = start.elapsed();

    println!("Elapsed time: {:?}", duration);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_test_sedgewick(path: String) -> Vec<String> {  //"data/test_small.txt"
    let data = read_lines(path);
    if data.is_err() {
        // todo panic
    }
    let lines = data.ok().expect("Error").flatten();
    let mut lines_data = &mut vec![];
    for line in lines {
        lines_data.push(line.trim().to_string())
    }
    lines_data.clone()
}
