use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use ndarray::s;
use crate::bag::Bag;
use crate::structure_xml::Network;
use crate::utils_graph::EdgeCapacityProduct;

#[derive(Debug, Clone, Copy)]
pub struct DirectedEdge {
    v: i32, //from
    w: i32, //to
    capacity: f64,
    cost: f64
}

impl PartialEq for DirectedEdge {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v && self.w == self.w
    }
}

impl Eq for DirectedEdge {}

impl Hash for DirectedEdge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.v.hash(state);
        self.w.hash(state);
    }
}

impl DirectedEdge {

    pub fn get_empty_edge(from: i32, to: i32) -> DirectedEdge {
        DirectedEdge {
            v: from,
            w: to,
            capacity: 0f64,
            cost: 0f64
        }
    }
    pub fn from(&self) -> i32 {
        return self.v
    }

    pub fn to(&self) -> i32 {
        return self.w
    }

    pub fn get_cost(&self) -> f64 { self.cost }

    pub fn get_capacity(&self) -> f64 { self.capacity }

    pub fn update_cost(&mut self, cost: f64) {
        self.cost = cost;
    }

    pub fn to_string(&self) {
        print!("{}->{} cost:{:.2}, capacity:{:.2}", self.v, self.w, self.cost, self.capacity)
    }
}

#[derive(Debug)]
pub struct EdgeWeightedDigraph {
    v_count: i32,     // количество вершин
    e_count: i32, // количество ребер
    adj: Option<Vec<Bag<RefCell<DirectedEdge>>>>,// списки смежности
    matrix: Option<Vec<Vec<f64>>>
}

impl EdgeWeightedDigraph {

    pub fn default_graph() -> EdgeWeightedDigraph {
        let mut graph = EdgeWeightedDigraph {
            v_count: 0,
            e_count: 0,
            adj: None,
            matrix: None
        };
        graph
    }

    pub fn graph_from_array_str(&mut self, mut array : Vec<String>) -> &mut Self{
        self.v_count = array[0].parse::<i32>().unwrap();
        self.e_count = array[1].parse::<i32>().unwrap();
        let mut adj = vec![];
        for _ in 0..self.v_count {
            adj.push(Bag::get_empty_bag());
        }
        for e in 2..array.len() {
            let arr : Vec<String>= array[e].split_whitespace().map(|s| s.to_string()).collect();
            let ver = arr[0].parse::<i32>().unwrap();
            let edg = arr[1].parse::<i32>().unwrap();
            let cost = arr[2].parse::<f64>().unwrap();
            let e = DirectedEdge {
                v: ver,
                w: edg,
                cost,
                capacity: 0f64
            };
            adj[e.from() as usize].add(RefCell::new(e));
            self.e_count += 1;
        }
        self.adj = Some(adj);
        self
    }

    pub fn graph_from_struct_xml(&mut self, network: &Network, map_index: &HashMap<String, i32>) -> &mut Self{
        let network_struct = network.get_network_structure();
        self.v_count = network_struct.get_node_count() as i32;
        self.e_count = network_struct.get_link_count() as i32;
        let mut adj = vec![];
        for _ in 0..self.v_count {
            adj.push(Bag::get_empty_bag());
        }
        let links = network_struct.get_links().get_vec_link();
        for link in links {
            let from = *map_index.get(&link.get_source()).unwrap();
            let to = *map_index.get(&link.get_target()).unwrap();
            let mut e = DirectedEdge{
                v: from,
                w: to,
                cost: link.get_cost(),
                capacity: link.get_capacity()
            };
            adj[e.from() as usize].add(RefCell::new(e));
            let mut _e = DirectedEdge {
                v: to,
                w: from,
                cost: link.get_cost(),
                capacity: link.get_capacity()
            };
            adj[_e.from() as usize].add(RefCell::new(_e));
            self.e_count += 1;
        }
        self.adj = Some(adj);
        self
    }

    pub fn graph_from_array_str_with_matrix(&mut self, mut array : Vec<String>) -> &mut Self{
        self.v_count = array[0].parse::<i32>().unwrap();
        self.e_count = array[1].parse::<i32>().unwrap();;
        let mut matrix = vec![vec![f64::MAX; self.v_count as usize]; self.v_count as usize];
        for e in 2..array.len() {
            let arr : Vec<String>= array[e].split_whitespace().map(|s| s.to_string()).collect();
            let ver = arr[0].parse::<i32>().unwrap();
            let edg = arr[1].parse::<i32>().unwrap();
            let cost = arr[2].parse::<f64>().unwrap();
            matrix[ver as usize][edg as usize] = cost;
        }
        self.matrix = Some(matrix);
        self
    }

    pub fn get_v_count(&self) -> i32 {
        self.v_count
    }

    pub fn get_e_count(&self) -> i32 {
        self.e_count
    }

    pub fn edge_list(&self, v:usize) -> &Bag<RefCell<DirectedEdge>> {
        let bag = self.adj.as_ref().unwrap();
        &bag[v]
    }

    /*pub fn edge_list_mut(&self, v: usize) -> RefMut<Bag<DirectedEdge>> {
        let adj_ref = self.adj.as_ref().unwrap().borrow_mut();
    }*/

    pub fn get_matrix_connectivity(&self) -> &Vec<Vec<f64>> {
         &self.matrix.as_ref().unwrap()
    }
}