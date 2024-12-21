use std::collections::HashMap;

use crate::delay_func_count::{count_delay_value, count_first_derivative, count_second_derivative};
use crate::graph::{DirectedEdge, EdgeWeightedDigraph};
use crate::utils_graph::{EdgeFlowCommodities, find_all_path};

pub struct ProjectionMethod {
    alpha: f64,
    p: f64,
    edge_commodity: HashMap<String, EdgeFlowCommodities>,
    path_commodity: HashMap<String, f64>,
    paths: HashMap<String, Vec<DirectedEdge>>,
}

impl ProjectionMethod {
    pub fn new(alpha: f64, p: f64) -> ProjectionMethod {
        ProjectionMethod {
            alpha,
            p,
            edge_commodity: HashMap::new(),
            path_commodity: HashMap::new(),
            paths: HashMap::new(),
        }
    }

    pub fn set_first_commodity_to_graph(
        &mut self,
        r_index: i32,
        flow: f64,
        path_shortest: &Vec<DirectedEdge>,
        graph_adj: &mut EdgeWeightedDigraph) {
        for mut edge in path_shortest {
            let to = edge.to();
            let from = edge.from();
            let key_edge = from.to_string() + "_" + &to.to_string();
            let check = self.edge_commodity.get_mut(&key_edge);
            match check {
                Some(mut x) => {
                    x.update_commodity_flow_x(&r_index, flow);
                    x.update_commodity_flow_y(&r_index, flow);
                }
                None => {
                    let mut commodities_flow = HashMap::new();
                    commodities_flow.insert(r_index, (flow, flow));
                    let edge_flows = EdgeFlowCommodities::new(edge.to_owned(), commodities_flow);
                    self.edge_commodity.insert(key_edge, edge_flows);
                }
            }
        }

        for (_, edge_flows) in self.edge_commodity.iter_mut() {
            let cost_total = edge_flows.get_total_flow_x();
            let mut edge = edge_flows.get_edge_mut();
            let cost = count_first_derivative(cost_total, edge.get_capacity(), self.p);
            graph_adj.update_edge(edge.from(), edge.to(), cost);
            edge.update_cost(cost);
        }
    }

    pub fn update_edge_flow(&mut self,
                            result_step: &HashMap<String, f64>,
                            commodity: i32,
                            graph_adj: &mut EdgeWeightedDigraph,
                            paths: &HashMap<String, Vec<DirectedEdge>>
    ) {
        let mut edges_for_update = HashMap::new();

        for (key, path_vec) in paths {
            for mut edge in path_vec {

                let mut flow = *result_step.get(key).expect("Ошибка");
                let key_edge = &(edge.from().to_string() + "_" + &edge.to().to_string());
                let check_edge = edges_for_update.get(key_edge).unwrap_or(&false);
                let check = self.edge_commodity.get_mut(key_edge);
                match check {
                    Some(mut x) => {
                        if *check_edge {
                            flow += x.get_total_flow_by_commodity(&commodity)
                        }
                        x.update_commodity_flow_x(&commodity, flow)
                    }
                    None => {
                        let mut commodities = HashMap::new();
                        commodities.insert(commodity, (flow, 0f64));
                        let edge_flows = EdgeFlowCommodities::new(edge.to_owned(), commodities);
                        self.edge_commodity.insert(key_edge.clone(), edge_flows);
                    }
                }
                edges_for_update.insert(key_edge.clone(), true);
            }
        }

        for (_, edge_flows) in self.edge_commodity.iter_mut() {
            let flow = edge_flows.get_total_flow_x();
            let mut edge = edge_flows.get_edge_mut();
            let capacity = edge.get_capacity();
            let cost = count_first_derivative(flow, capacity, self.p);
            graph_adj.update_edge(edge.from(), edge.to(), cost);
            edge.update_cost(cost);
        }
    }


    pub fn update_edge_flow_y(&mut self,
                            commodity: i32,
                            flow: f64,
                            edges: &Vec<DirectedEdge>
    ) {
        for mut edge in edges {
            let key_edge = edge.from().to_string() + "_" + &edge.to().to_string();
            let check = self.edge_commodity.get_mut(&key_edge);
            match check {
                Some(mut x) => {
                    x.update_commodity_flow_y(&commodity, flow)
                }
                None => {
                    let mut commodities = HashMap::new();
                    commodities.insert(commodity, (0f64, flow));
                    let edge_flows = EdgeFlowCommodities::new(edge.to_owned(), commodities);
                    self.edge_commodity.insert(key_edge, edge_flows);
                }
            }
        }
    }

    fn get_all_path_from_source_to_target(&mut self,
        graph_adj: &mut EdgeWeightedDigraph,
        source: i32,
        target: i32) {
        self.paths = find_all_path(graph_adj, source, target);
    }

    pub fn get_d_k_p (&self, commodity: &i32, path_edges: &Vec<DirectedEdge>) -> f64 {
        path_edges.iter().map(|e| self.get_derivative_one_edge(commodity, e)).sum()
    }

    pub fn get_delay_value_x(&self) -> f64 {
        self.edge_commodity.iter().map(|(_, val)| self.delay(val.get_total_flow_x(), val.get_edge().get_capacity())).sum()
    }

    pub fn get_delay_value_y(&self) -> f64 {
        self.edge_commodity.iter().map(|(_, val)| self.delay(val.get_total_flow_y(), val.get_edge().get_capacity())).sum()
    }

    pub fn get_delay_gradient(&self) -> Vec<f64> {
        let mut sorted_keys: Vec<_> = self.edge_commodity.keys().cloned().collect();
        sorted_keys.sort(); // Сортируем ключи

        sorted_keys
            .into_iter()
            .filter_map(|key| self.edge_commodity.get(&key))
            .map(|e| count_first_derivative(e.get_total_flow_x(), e.get_edge().get_capacity(), self.p))
            .collect::<Vec<f64>>()
    }

    pub fn get_total_flow_vector_x(&self) -> Vec<f64> {
        let mut sorted_keys: Vec<_> = self.edge_commodity.keys().cloned().collect();
        sorted_keys.sort(); // Сортируем ключи

        sorted_keys
            .into_iter()
            .filter_map(|key| self.edge_commodity.get(&key))
            .map(|e| e.get_total_flow_x())
            .collect()
    }

    pub fn get_total_flow_vector_y(&self) -> Vec<f64> {
        let mut sorted_keys: Vec<_> = self.edge_commodity.keys().cloned().collect();
        sorted_keys.sort(); // Сортируем ключи

        sorted_keys
            .into_iter()
            .filter_map(|key| self.edge_commodity.get(&key))
            .map(|e| e.get_total_flow_y())
            .collect()
    }

    pub fn get_d_k_p_new (&self, x: f64, path_edges: &Vec<DirectedEdge>) -> f64 {
        path_edges.iter().map(|e| count_first_derivative(x, e.get_capacity(), self.p)).sum()
    }

    fn get_derivative_one_edge (&self, commodity: &i32, edge: &DirectedEdge) -> f64 {
        let key = edge.from().to_string() + "_" + &edge.to().to_string();
        let edge_flows = self.edge_commodity.get(&key);
        match edge_flows {
            Some(com_flow) => {
                let flow = com_flow.get_total_flow_x();
                count_first_derivative(flow, edge.get_capacity(), self.p)
            },
            None => 0f64
        }
    }

    fn delay(&self, flow: f64, capacity: f64) -> f64 {
        count_delay_value(flow, capacity, self.p)
    }

    pub fn get_h_k_p(&self, commodity: &i32, path_edges: &Vec<DirectedEdge>) -> f64 {
        path_edges.iter().map(|e| self.get_derivative_two_edge(commodity, e)).sum()
    }

    fn get_derivative_two_edge (&self, commodity: &i32, edge: &DirectedEdge) -> f64 {
        let key = edge.from().to_string() + "_" + &edge.to().to_string();
        let edge_flows = self.edge_commodity.get(&key);
        match edge_flows {
            Some(com_flow) => {
                let flow = com_flow.get_total_flow_by_commodity(commodity);
                count_second_derivative(flow, edge.get_capacity(), self.p)
            },
            None => 0f64
        }
    }

    pub fn get_alpha(&self) -> f64{
        self.alpha
    }
}