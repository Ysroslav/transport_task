use std::fmt;
use ndarray::Array2;
use crate::bag::Bag;

#[derive(Debug, Clone, Copy)]
pub struct DirectedEdge {
    v: i32,
    w: i32,
    weight: f32
}

impl DirectedEdge {
    pub fn from(&self) -> i32 {
        return self.v
    }

    pub fn to(&self) -> i32 {
        return self.w
    }

    pub fn get_weight(&self) -> f32 { self.weight }

    pub fn to_string(&self) {
        print!("{}->{}:{:.2}", self.v, self.w, self.weight)
    }
}

#[derive(Debug)]
pub struct EdgeWeightedDigraph {
    v_count: i32,     // количество вершин
    e_count: i32,     // количество ребер
    adj: Option<Vec<Bag<DirectedEdge>>>,// списки смежности
    matrix: Option<Vec<Vec<f32>>>
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
            let weight = arr[2].parse::<f32>().unwrap();
            let e = DirectedEdge{
                v: ver,
                w: edg,
                weight
            };
            adj[e.from() as usize].add(e);
            self.e_count += 1;
        }
        self.adj = Some(adj);
        self
    }

    pub fn graph_from_array_str_with_matrix(&mut self, mut array : Vec<String>) -> &mut Self{
        self.v_count = array[0].parse::<i32>().unwrap();
        self.e_count = array[1].parse::<i32>().unwrap();;
        let mut matrix = vec![vec![f32::MAX; self.v_count as usize]; self.v_count as usize];
        for e in 2..array.len() {
            let arr : Vec<String>= array[e].split_whitespace().map(|s| s.to_string()).collect();
            let ver = arr[0].parse::<i32>().unwrap();
            let edg = arr[1].parse::<i32>().unwrap();
            let weight = arr[2].parse::<f32>().unwrap();
            matrix[ver as usize][edg as usize] = weight;
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

    pub fn edge_list(&self, v:usize) -> &Bag<DirectedEdge> {
        let bag = self.adj.as_ref().unwrap();
        &bag[v]
    }

    pub fn get_matrix_connectivity(&self) -> &Vec<Vec<f32>> {
         &self.matrix.as_ref().unwrap()
    }
}