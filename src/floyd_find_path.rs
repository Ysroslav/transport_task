use std::collections::HashMap;
use crate::graph::{DirectedEdge, EdgeWeightedDigraph};

pub struct FloydSP {
    dist_to: Vec<Vec<f64>>,
    next: Vec<Vec<usize>>
}

impl FloydSP {

    pub fn floyd(graph: &EdgeWeightedDigraph) -> FloydSP {
        let vertexes = graph.get_v_count() as usize;
        let mut dij = FloydSP {
            dist_to: vec![vec![0.0f64; vertexes]; vertexes],
            next: vec![vec![0usize; vertexes]; vertexes]
        };

        let vertexes = graph.get_v_count() as usize;
        let mut matrix = graph.get_matrix_connectivity().clone();

        for i in 0..vertexes {
            for j in 0..vertexes {
                if matrix[i][j] != 0f64 && i != j {
                    dij.next[i][j] = j;
                }
            }
        }

        for i in (0..vertexes){
            for u in (0..vertexes) {
                for v in (0..vertexes) {
                    if matrix[u][i] + matrix[i][v] < matrix[u][v] {
                        matrix[u][v] = matrix[u][i] + matrix[i][v];
                        dij.next[u][v] = dij.next[u][i];
                        dij.dist_to[u][v] = matrix[u][v];
                    }
                }
            }
        }

        dij
    }

    pub fn dist_to(&self, u : usize, v: usize) -> f64 {
        self.dist_to[u][v]
    }

    pub fn has_path_to(&self, u : usize, v: usize) -> bool {
        self.next[u][v] > 0 && self.next[u][v] < i32::MAX as usize
    }

    pub fn path_to(&self, u : usize, v: usize) -> Option<Vec<usize>> {
        if !self.has_path_to(u, v) {
            return None
        }
        let mut result: Vec<usize> = vec![];
        let mut i = u;
        while i != v {
            result.push(i);
            i = self.next[i][v]
        }
        result.push(v);
        Some(result)
    }
}