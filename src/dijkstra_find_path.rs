use std::collections::HashMap;
use crate::index_min_pq::IndexMinPQ;
use crate::graph::EdgeWeightedDigraph;
use crate::graph::DirectedEdge;

pub struct DijkstraSP {
    edge_to: HashMap<i32, DirectedEdge>,
    dist_to: Vec<f64>,
    pq: IndexMinPQ
}

impl DijkstraSP {

    pub fn dijkstra(graph: &mut EdgeWeightedDigraph, s:i32) -> DijkstraSP {
        let mut dij = DijkstraSP {
            edge_to: HashMap::new(),
            dist_to: vec![f64::INFINITY; graph.get_v_count() as usize],
            pq: IndexMinPQ::get_index_from_size(graph.get_v_count())
        };
        dij.dist_to[s as usize] = 0.0;
        dij.pq.insert(s as usize, 0.0);

        while !dij.pq.is_empty() {
            let v = dij.pq.del_min();
            Self::relax(&mut dij, &graph, v)
        }
        dij
    }

    fn relax(dij: &mut DijkstraSP, graph: &EdgeWeightedDigraph, v: usize){
        for gr in graph.edge_list(v).iter() {
            let w = gr.to() as usize;
            if dij.dist_to[w] > dij.dist_to[v] + gr.get_cost() {
                dij.dist_to[w] = dij.dist_to[v] + gr.get_cost();
                dij.edge_to.insert(w as i32, *gr);
                if dij.pq.contains(w) {
                    dij.pq.change(w, dij.dist_to[w]);
                    continue;
                }
                dij.pq.insert(w, dij.dist_to[w])
            }
        }
    }

    pub fn dist_to(&self, v: usize) -> f64 {
        self.dist_to[v]
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v] >= 0.0 && self.dist_to[v] < f64::INFINITY // предусмотреть что self.dist_to[v] равен 0
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<DirectedEdge>> {
        if !self.has_path_to(v) {
            return None
        }
        let mut result: Vec<DirectedEdge> = vec![];
        let mut i = v as i32;
        /*let mut e = self.edge_to.get(&i);
        while e.is_none() {
            result.push(*e.unwrap());
            i = dgraph.from();
        }*/
        for (_, dgraph) in &self.edge_to {
            let e = self.edge_to.get(&i);
            if e.is_none() {
                break;
            }
            result.push(*e.unwrap());
            i = e.unwrap().from();
        }
        Some(result)
    }
}

pub struct DijkstraAllPairsSP {
    all: Vec<DijkstraSP>
}

impl DijkstraAllPairsSP {

    pub fn get_all_pairs(graph: &mut EdgeWeightedDigraph) -> Self {
        let mut dij = DijkstraAllPairsSP {
            all: vec![]
        };
        let mut v = 0;
        while v < graph.get_v_count() {
            let d = DijkstraSP::dijkstra(graph, v);
            dij.all.push(d);
            v += 1;
        }
        dij
    }
}

