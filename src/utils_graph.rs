use std::cell::{RefCell, RefMut};
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::mem::take;
use std::ops::Deref;

use crate::graph::{DirectedEdge, EdgeWeightedDigraph};

#[derive(Debug, Clone)]
pub struct EdgeCapacityProduct {
    from: i32,
    to: i32,
    capacity: f64,
    products: RefCell<HashMap<i32, f64>>
}

impl EdgeCapacityProduct {

    pub fn get_products(&self) -> HashMap<i32, f64> {
        self.products.borrow().clone()
    }

    pub fn update_value(&self, key: i32, value: f64) {
        self.products.borrow_mut().insert(key, value);
    }

    pub fn get_products_by_commodity(&self, k: &i32) -> f32 {
        0f32
        //todo переделать после распределения продуктов
        //self.products.get(k).expect("")
    }

    pub fn get_capacity(&self) -> f64 {
        self.capacity
    }

    pub fn get_current(&self) -> i32 {
        self.to
    }
}

impl PartialEq for EdgeCapacityProduct {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == self.to
    }
}

impl Eq for EdgeCapacityProduct {}

impl Hash for EdgeCapacityProduct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}

pub fn find_all_path(
    graph: &EdgeWeightedDigraph,
    from: i32,
    to: i32,
) -> HashMap<String, Vec<DirectedEdge>> {
    let mut paths = HashMap::new();
    let mut current_path = Vec::new();
    let mut visited = HashSet::new();
    let edge =  &mut DirectedEdge::get_empty_edge(from, from);
    dfs(edge, to, &mut visited, &mut current_path, &mut paths, graph);
    paths
}

fn dfs(
    edge: &DirectedEdge,
    end: i32,
    visited: &mut HashSet<i32>,
    current_path: &mut Vec<DirectedEdge>,
    paths: &mut HashMap<String, Vec<DirectedEdge>>,
    graph: &EdgeWeightedDigraph,
) {
    let to = edge.to();

    visited.insert(to);
    current_path.push(edge.clone()); // Push a clone of the current edge

    if to == end {
        // Store a copy of the current path
        paths.insert(vec_to_str(current_path), current_path.clone());
    } else {
        // Iterate over all neighbors
        let mut edges = &graph.edge_list(to as usize);
        for gr in edges.iter() {
            if !visited.contains(&gr.to()) {
                dfs(gr, end, visited, current_path, paths, graph);
            }
        }
    }

    // Backtrack
    current_path.pop();
    visited.remove(&to);
}

fn vec_to_str(v: &Vec<DirectedEdge>) -> String {
    v.iter()
        .map(|n| n.to().to_string())
        .collect::<Vec<String>>()
        .join("_")
}

#[derive(Debug)]
pub struct EdgeFlowCommodities {
    edge: DirectedEdge,
    commodities: HashMap<i32, f64>
}

impl EdgeFlowCommodities {

    pub fn new(edge: DirectedEdge, commodities: HashMap<i32, f64>) -> EdgeFlowCommodities {
        EdgeFlowCommodities {
            edge,
            commodities
        }
    }

    pub fn get_edge(&self) -> DirectedEdge {
        self.edge
    }

    pub fn get_edge_mut(&mut self) -> &mut DirectedEdge {
        &mut self.edge
    }

    pub fn update_commodity(&mut self, commodity: &i32, flow: f64){
        let commodity_flow = self.commodities.get(commodity);
        match commodity_flow {
            Some(x) => {
                let f = x + flow;
                self.commodities.insert(*commodity, f);
            },
            None => {
                panic!("Продукт не найден")
            }
        }
    }

    pub fn get_total_flow_by_commodity(&self, commodity: &i32) -> f64 {
        let commodity_flow = self.commodities.get(commodity);
        match commodity_flow {
            Some(x) => {
                *x
            },
            None => {
                panic!("Продукт не найден")
            }
        }
    }

    pub fn get_total_flow(&self) -> f64 {
        self.commodities.values().sum()
    }

}

