use std::collections::HashMap;

use crate::delay_func_count::{count_first_derivative, count_second_derivative};
use crate::dijkstra_find_path::DijkstraSP;
use crate::graph::{DirectedEdge, EdgeWeightedDigraph};
use crate::utils_graph::{EdgeFlowCommodities, find_all_path};
use crate::utils_graph::vec_edge_to_str;
use crate::utils_graph::symmetric_difference;

pub struct ProjectionMethod {
    n: i32,
    alpha: f64,
    edge_commodity: HashMap<String, EdgeFlowCommodities>,
    path_commodity: HashMap<String, f64>,
    paths: HashMap<String, Vec<DirectedEdge>>,
}

impl ProjectionMethod {
    pub fn new(n: i32, alpha: f64) -> ProjectionMethod {
        ProjectionMethod {
            n,
            alpha,
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
                    x.update_commodity(&r_index, flow)
                }
                None => {
                    let mut commodities_flow = HashMap::new();
                    commodities_flow.insert(r_index, flow);
                    let edge_flows = EdgeFlowCommodities::new(edge.to_owned(), commodities_flow);
                    self.edge_commodity.insert(key_edge, edge_flows);
                }
            }
        }

        for (_, edge_flows) in self.edge_commodity.iter_mut() {
            let cost_total = edge_flows.get_total_flow();
            let mut edge = edge_flows.get_edge_mut();
            let cost = count_first_derivative(cost_total, edge.get_capacity());
            graph_adj.update_edge(edge.from(), edge.to(), cost);
            edge.update_cost(cost);
        }
    }

    pub fn update_edge_flow(&mut self,
                            result_step: &HashMap<String, f64>,
                            commodity: i32,
                            graph_adj: &mut EdgeWeightedDigraph) {
        let mut edge_commodity: HashMap<String, EdgeFlowCommodities> = HashMap::new();
        for (key, path_vec) in &self.paths {
            for mut edge in path_vec {
                let flow = result_step.get(key).expect("Ошибка");
                let key_edge = edge.from().to_string() + "_" + &edge.to().to_string();
                let check = edge_commodity.get_mut(&key_edge);
                match check {
                    Some(mut x) => {
                        x.update_commodity(&commodity, *flow)
                    }
                    None => {
                        let mut commodities = HashMap::new();
                        commodities.insert(commodity, *flow);
                        let edge_flows = EdgeFlowCommodities::new(edge.to_owned(), commodities);
                        edge_commodity.insert(key_edge, edge_flows);
                    }
                }
            }
        }

        for (_, edge_flows) in edge_commodity.iter_mut() {
            let flow = edge_flows.get_total_flow();
            let mut edge = edge_flows.get_edge_mut();
            let capacity = edge.get_capacity();
            let cost = count_first_derivative(flow, capacity);
            graph_adj.update_edge(edge.from(), edge.to(), cost);
            edge.update_cost(cost);
        }
        self.edge_commodity = edge_commodity
    }

    fn get_all_path_from_source_to_target(&mut self,
        graph_adj: &mut EdgeWeightedDigraph,
        source: i32,
        target: i32) {
        self.paths = find_all_path(graph_adj, source, target);
    }

    pub fn count_by_projection_method(
        &mut self,
        graph_adj: &mut EdgeWeightedDigraph,
        source: i32,
        target: i32,
        r_k: f64,
        r_index: i32
    ) -> HashMap<String, f64> {

        let mut map_result = HashMap::new();

        //запускаем алгоритм Дейкстры, для определения кратчайшего пути
        let mut sp = DijkstraSP::dijkstra(graph_adj, source);
        let path_s = &sp.path_to(target as usize).expect("Путь не найден");
        let key_path_s = &vec_edge_to_str(path_s);

        // устанавливаем поток на найденный кратчайший путь и обновляем ребра графа расчитавая cost
        self.set_first_commodity_to_graph(r_index, r_k, path_s, graph_adj);

        // находим новый кратчайший путь после обновления ребр первоначального пути
        sp = DijkstraSP::dijkstra(graph_adj, source);

        let path_s_second = &sp.path_to(target as usize).expect("Путь не найден");

        let key_path_s_second = &vec_edge_to_str(path_s_second);
        if key_path_s == key_path_s_second {
            map_result.insert(key_path_s.clone(), r_k);
            return map_result;
        }

        // добавляем изначальный путь в список активных путей
        self.paths.insert(key_path_s.clone(), path_s.clone());
        self.path_commodity.insert(key_path_s.clone(), r_k);

        // стартуем расчет
        for _ in 0..self.n {  //todo заменить на невязку

            let mut result = 0f64;

            //расчитыаем d_kp для кратчайшего пути
            let d_kp_s = self.get_d_k_p(&r_index, &path_s_second, &self.edge_commodity);

            //определяем x_kp^(t+1) для каждого пути кроме кратчайшего
            for (key, value) in &self.paths {
                let d_kp_i: _ = self.get_d_k_p(&r_index, value, &self.edge_commodity);
                let lk_p = symmetric_difference(value.clone(), path_s.clone());
                let h_kp_i = self.get_h_k_p(&r_index, &lk_p, &self.edge_commodity);
                let x_k_p_t = self.path_commodity.get(key).expect("Ошибка, начальное значение потока не найдено");

                //вычисляем x_k_p
                let x_k_p_t_1 = &f64::max(0f64, x_k_p_t - self.alpha * ((1f64 / h_kp_i) * (d_kp_i - d_kp_s)));
                if x_k_p_t_1 == &0f64 {
                    // todo удаляем путь из активных путей
                    continue;
                }
                map_result.insert(key.clone(), *x_k_p_t_1);
                self.path_commodity.insert(key.clone(), *x_k_p_t_1); // обновляем поток для следующего шага
                result += x_k_p_t_1;
            }
            let x_k_p_s_t = r_k - result; // рассчитываем поток для кратчайшего пути
            map_result.insert(key_path_s.clone(), x_k_p_s_t); // сохраняем результат

            // обновляем общий поток и cost по ребрам
            self.update_edge_flow(&map_result, r_index, graph_adj);
        }
        map_result
    }

    fn get_d_k_p (&self, commodity: &i32, path_edges: &Vec<DirectedEdge>, edges: &HashMap<String, EdgeFlowCommodities>) -> f64 {
        path_edges.iter().map(|e| self.get_derivative_one_edge(commodity, e, edges)).sum()
    }

    fn get_derivative_one_edge (&self, commodity: &i32, edge: &DirectedEdge, edges: &HashMap<String, EdgeFlowCommodities>) -> f64 {
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

    fn get_h_k_p(&self, commodity: &i32, path_edges: &Vec<DirectedEdge>, edges: &HashMap<String, EdgeFlowCommodities>) -> f64 {
        path_edges.iter().map(|e| self.get_derivative_two_edge(commodity, e, edges)).sum()
    }

    fn get_derivative_two_edge (&self, commodity: &i32, edge: &DirectedEdge, edges: &HashMap<String, EdgeFlowCommodities>) -> f64 {
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
}