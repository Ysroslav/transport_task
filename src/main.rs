use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::iter::Flatten;
use std::mem::take;
use std::ops::Deref;
use std::path::Path;
use std::time::Instant;
use nalgebra::Matrix3;
use ndarray::{arr1, arr2, Array1};
use crate::delay_func_count::{count_delay_value, count_first_derivative, count_second_derivative};
use crate::dijkstra_find_path::DijkstraSP;
use crate::floyd_find_path::FloydSP;
use crate::graph::{DirectedEdge, EdgeWeightedDigraph};
use crate::parser_xml::parse_xml_to_structure;
use crate::utils_graph::{EdgeCapacityProduct, EdgeFlowCommodities, find_all_path};

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_test_sedgewick(path: String) ->  Vec<String> {  //"data/test_small.txt"
    let data = read_lines(path);
    if data.is_err() {
        // todo panic
    }
    let lines =  data.ok().expect("Error").flatten();
    let mut lines_data = &mut vec![];
    for line in lines {
        lines_data.push(line.trim().to_string())
    }
    lines_data.clone()
}



fn main() {
    let start = Instant::now();

    let p = parse_xml_to_structure("C:\\Users\\Dell\\mipt\\abilene.xml");

    let mut point_index = HashMap::new();

    let nodes = p.get_network_structure().get_nodes().get_node_vec();
    for (i, node) in nodes.iter().enumerate() {
        point_index.insert(node.get_id(), i as i32);
    }

    let commodities = p.get_demands().get_demand_vec();


    let N = 1000;

    let alpha:f64 = 0.99;
    //определяем граф
    let mut g = EdgeWeightedDigraph::default_graph();
    let mut graph_adj = EdgeWeightedDigraph::graph_from_struct_xml(& mut g, &p, &point_index);


    for (index, commodity) in commodities.iter().enumerate() {
        let source: &i32 = point_index.get(&commodity.get_source()).unwrap();
        let target: &i32 = point_index.get(&commodity.get_target()).unwrap();
        let r_k = commodity.get_demand_vale();
        let r_index = index as i32;

        //находим все пути
        let mut paths = find_all_path(graph_adj, *source, *target);

        let count_p = paths.len() as f64;

        //определяем сколько установим поток на каждый путь - первоначаьная установка todo добавить другие потоки
        let x_0_p = r_k / (count_p);
        let mut edge_commodity:HashMap<String, EdgeFlowCommodities> = HashMap::new();
        let mut path_commodity = HashMap::new();
        for (key, path_vec) in &paths{
            for mut edge_ref in path_vec {
                let mut edge = edge_ref.borrow_mut();
                let to = edge.to();
                let from = edge.to();
                let capacity = edge.get_capacity();
                let cost = count_first_derivative(x_0_p, capacity);
                edge.update_cost(cost);
                let key_edge = from.to_string() + "_" + &to.to_string();
                let check = edge_commodity.get_mut(&key_edge);
                match check {
                    Some(mut x) => {
                        x.update_commodity(&r_index, x_0_p)
                    },
                    None => {
                        let mut commodities_flow = HashMap::new();
                        commodities_flow.insert(r_index, x_0_p);
                        let edge_flows = EdgeFlowCommodities::new(edge.to_owned(), commodities_flow);
                        edge_commodity.insert(key_edge, edge_flows);
                    }
                }
            }
            path_commodity.insert(key, x_0_p);
        }

        /*for (_, edge_flows) in edge_commodity.iter_mut() {
            let mut edge = edge_flows.get_edge_mut();
            let cost = count_first_derivative(x_0_p, edge.get_capacity());
            edge.update_cost(cost);
        }*/

        // старт расчета
        //запуск алгоритма Дейкстры
        let sp = DijkstraSP::dijkstra(graph_adj, *source);
        let key_path_s = &vec_edge_to_str(&sp.path_to(*target as usize).expect("Путь не найден"));

        let path_s = paths.get(key_path_s).unwrap().clone();
        //запуск метода проекции
        for i in 0..N {
            let mut result = 0f64;
            let mut map_result = HashMap::new();
            let d_kp_s = get_d_k_p(&r_index, &path_s, &edge_commodity);
            for (key, value) in &paths {
                if key == key_path_s {
                    continue;
                }
                let d_kp_i: _ = get_d_k_p(&r_index, value, &edge_commodity);
                let lk_p = symmetric_difference(value.clone(), path_s.clone());
                let h_kp_i = get_h_k_p(&r_index, &lk_p, &edge_commodity);
                let x_k_p_t = path_commodity.get(key).expect("Ошибка, начальное значение потока не найдено");
                //вычисляем x_k_p
                let x_k_p_t_1 = &f64::max(0f64, x_k_p_t - alpha * ((1f64 / h_kp_i) * (d_kp_i - d_kp_s)));
                map_result.insert(key.clone(), *x_k_p_t_1);
                //path_commodity.insert(key, *x_k_p_t_1);
                result += x_k_p_t_1;
            }
            let x_k_p_s_t = r_k - result;
            map_result.insert(key_path_s.clone(), x_k_p_s_t);
            edge_commodity = update_edge_flow(&paths, &map_result, r_index);
        }
    }
    let duration = start.elapsed();

    println!("Elapsed time: {:?}", duration);
}

fn update_edges(x: &Vec<f64>, mut edges: Vec<EdgeCapacityProduct>, product: i32) {
    for (i, mut edge) in edges.iter().enumerate() {
        if edge.get_current() == 0 {
            continue;
        }
        edge.update_value(product, x[i])
    }
}

fn vec_edge_to_str(edges: &Vec<DirectedEdge>) -> String{
    let mut result = String::new();
    result = edges[0].to().to_string();
    for i in 1..edges.len() {
        result = edges[i].to().to_string() + "_" + &result;
    }
    result = edges[edges.len() - 1].from().to_string() + "_" + &result;
    result
}

fn count_d_kp_edge_1(edge: &EdgeCapacityProduct) -> f64 {
    let x_0_j = edge.get_products().values().sum();
    let c_j = edge.get_capacity();
    count_first_derivative(x_0_j, c_j)
}

fn count_d_kp_edge_2(edge: &EdgeCapacityProduct) -> f64 {
    let x_0_j = edge.get_products().values().sum();
    let c_j = edge.get_capacity();
    count_second_derivative(x_0_j, c_j)
}

fn symmetric_difference<T: Eq + std::hash::Hash>(vec1: Vec<RefCell<T>>, vec2: Vec<RefCell<T>>) -> Vec<T> {
    let vec1_inner: Vec<T> = vec1.into_iter().map(|v| v.into_inner()).collect();
    let vec2_inner: Vec<T> = vec2.into_iter().map(|v| v.into_inner()).collect();
    let combined: HashSet<_> = vec1_inner.into_iter().chain(vec2_inner).collect();
    combined.into_iter().collect()
}

/*fn symmetric_difference<T: Eq + std::hash::Hash>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let combined: HashSet<_> = vec1.into_iter().chain(vec2).collect();
    combined.into_iter().collect()
}*/

fn get_d_k_p (commodity: &i32, path_edges: &Vec<RefCell<DirectedEdge>>, edges: &HashMap<String, EdgeFlowCommodities>) -> f64 {
    path_edges.iter().map(|e| get_derivative_one_edge(commodity, &e.borrow().clone(), edges)).sum()
}

fn get_derivative_one_edge (commodity: &i32, edge: &DirectedEdge, edges: &HashMap<String, EdgeFlowCommodities>) -> f64 {
    let key = edge.from().to_string() + "_" + &edge.to().to_string();
    let edge_flows = edges.get(&key);
    match edge_flows {
        Some(com_flow) => {
            let flow = com_flow.get_total_flow_by_commodity(commodity);
            count_first_derivative(flow, edge.get_capacity())
        },
        None => 0f64
    }
}

fn get_h_k_p(commodity: &i32, path_edges: &Vec<DirectedEdge>, edges: &HashMap<String, EdgeFlowCommodities>) -> f64 {
    path_edges.iter().map(|e| get_derivative_two_edge(commodity, e, edges)).sum()
}

fn get_derivative_two_edge (commodity: &i32, edge: &DirectedEdge, edges: &HashMap<String, EdgeFlowCommodities>) -> f64 {
    let key = edge.from().to_string() + "_" + &edge.to().to_string();
    let edge_flows = edges.get(&key);
    match edge_flows {
        Some(com_flow) => {
            let flow = com_flow.get_total_flow_by_commodity(commodity);
            count_second_derivative(flow, edge.get_capacity())
        },
        None => 0f64
    }
}

fn update_edge_flow (
    paths: &HashMap<String, Vec<RefCell<DirectedEdge>>>,
    result: &HashMap<String, f64>,
    commodity: i32) -> HashMap<String, EdgeFlowCommodities> {
    let mut edge_commodity:HashMap<String, EdgeFlowCommodities > = HashMap::new();
    for (key, path_vec) in paths{
        for mut edge_ref in path_vec {
            let flow = result.get(key).expect("Ошибка");
            let edge = edge_ref.borrow();
            let key_edge = edge.from().to_string() + "_" + &edge.to().to_string();
            let check = edge_commodity.get_mut(&key_edge);
            match check {
                Some(mut x) => {
                    x.update_commodity(&commodity, *flow)
                },
                None => {
                    let mut commodities = HashMap::new();
                    commodities.insert(commodity, *flow);
                    let edge_flows = EdgeFlowCommodities::new(edge.to_owned(), commodities);
                    edge_commodity.insert(key_edge, edge_flows);
                }
            }
        }
    }

    /*for (_, edge_flows) in edge_commodity.iter_mut() {
        let flow = edge_flows.get_total_flow_by_commodity(&commodity);
        let mut edge = edge_flows.get_edge_mut();
        let capacity = edge.get_capacity();
        let cost = count_first_derivative(flow, capacity);
        edge.update_cost(cost);
    }*/
    edge_commodity
}
