use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Deref;
use std::path::Path;
use std::time::Instant;
use ndarray::Array1;

use crate::dijkstra_find_path::DijkstraSP;
use crate::graph::EdgeWeightedDigraph;
use crate::parser_xml::parse_xml_to_structure;
use crate::projection_method::ProjectionMethod;
use crate::structure_xml::Demand;
use crate::utils_graph::{symmetric_difference, vec_edge_to_str};

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

    let network = parse_xml_to_structure("C:\\Users\\Dell\\mipt\\abilene.xml");

    let mut point_index = HashMap::new();

    // индексируем вершины графа, для более быстрого расчета алгоритма Дейкстры
    let nodes = network.get_network_structure().get_nodes().get_node_vec();
    for (i, node) in nodes.iter().enumerate() {
        point_index.insert(node.get_id(), i as i32);
    }

    let commodities = network.get_demands().get_demand_vec();

    // данные для расчета метода проекции
    let alpha: f64 = 0.065;
    let p = 0.99;
    let mut LB = 0f64;
    let epsilon: f64 = 0.0001;

    //определяем сеть, каждое ребро добавляется два раза, в одну сторону и в другую
    let mut g = EdgeWeightedDigraph::default_graph();
    let graph_adj = EdgeWeightedDigraph::graph_from_struct_xml(&mut g, &network, &point_index);

    // map для хранения результата индекс commodity на пару path на размер потока на соответсвующем path
    let mut result_x = HashMap::new();

    // map для хранения кратчайшего пути для соответвующего commodity, индекс commodity на вектор ребер полученных из сети
    let mut paths_shortest = HashMap::new();
    let mut projection_handler = ProjectionMethod::new(alpha, p);

    // map для хранения активных path для соответвующего commodity, индекс commodity на map path -> список ребер
    let mut active_paths = HashMap::new();

    // определям кратчайшие пути для каждого commodity и ставим потоки на данные пути, после этого пересчитываем
    // затраты на каждом ребре, которые входят в кратчайшие пути
    for (index, commodity) in commodities.iter().enumerate() {
        let source: &i32 = point_index.get(&commodity.get_source()).unwrap();
        let target: &i32 = point_index.get(&commodity.get_target()).unwrap();
        let r_k = commodity.get_demand_vale();
        let r_index = index as i32;

        let mut sp = DijkstraSP::dijkstra(graph_adj, *source);
        // расчет кратчайшего маршрута, через алгоритм Дейкстры
        let path_s = sp.path_to(*target as usize).expect("Путь не найден");
        let key_path_s = &vec_edge_to_str(&path_s);

        let mut active_paths_commodity = HashMap::new();
        active_paths_commodity.insert(key_path_s.clone(), path_s.clone());
        active_paths.insert(r_index, active_paths_commodity);

        // устанавливаем поток на найденный кратчайший путь и обновляем ребра графа расчитавая cost
        projection_handler.set_first_commodity_to_graph(r_index, r_k, &path_s, graph_adj);

        // сохраняем кратчайший путь
        paths_shortest.insert(r_index, path_s);
    }

    println!("{}", projection_handler.get_delay_value_x());

    // задаем векторы и переменные для расчета остановки алгоритма
    // остановка расчитывается по формуле статья Adam Ouorou для метода Flow Deviation
    let mut y_j_t: Vec<f64> = Vec::new();
    let mut x_j_t: Vec<f64> = Vec::new();
    let mut delay_value_t = 0f64;
    let mut grad = Vec::new();

        // запуск работы метода, проходимся по каждому commodity,
        // определяем новый кратчайший маршрут, если он совпадает с первоначальным, то считаем что маршрут для этого commodity определен и переходим к следующему
        // если маршрут не совпадает добавлем его в список активных путей, и запускаем метод PM

    delay_value_t = projection_handler.get_delay_value_x();

    loop {
         // значение функции
        let x_j_t_value = projection_handler.get_total_flow_vector_x(); // значение потока на всех ребрах
        x_j_t = x_j_t_value;
        let y_j_t_value = projection_handler.get_total_flow_vector_y(); // значение потока на всех ребрах, если мы весь поток ставим на кратчайший маршрут
        y_j_t = y_j_t_value;
        let grad_value = projection_handler.get_delay_gradient(); // значение градиента функции после распределения потока по ребрам
        grad = grad_value;
        for (index, commodity) in commodities.iter().enumerate() {
            let source: &i32 = point_index.get(&commodity.get_source()).unwrap();
            let target: &i32 = point_index.get(&commodity.get_target()).unwrap();
            let r_index = index as i32;

            let mut method_step_commodity = HashMap::new();

            loop {

                // опеределяем новый кратчайший маршрут
                let mut sp = DijkstraSP::dijkstra(graph_adj, *source);
                let path_s = sp.path_to(*target as usize).expect("Путь не найден");
                let key_path_s = &vec_edge_to_str(&path_s);

                //проверяем совпадает ли он с первоначальным путем
                let path_old = paths_shortest.get(&r_index).unwrap();
                let key_path_old = &vec_edge_to_str(path_old);

                method_step_commodity.insert(key_path_old.clone(), commodity.get_demand_vale());

                if key_path_s == key_path_old {
                    // если путь совпадает, сохраняем результат и берем следующий commodity
                    result_x.insert(r_index, method_step_commodity);
                    break;
                }

                // добавляем найденый путь в список активных путей
                let mut active_paths_commodity = active_paths.get_mut(&r_index).unwrap();
                active_paths_commodity.insert(key_path_s.clone(), path_s.clone());

                let d_kp_s = projection_handler.get_d_k_p(&r_index, &path_s);

                let mut result = 0f64;

                //определяем x_kp^(t+1) для каждого пути кроме кратчайшего
                for (key, value) in active_paths_commodity.clone() {
                    if &key == key_path_s {
                        projection_handler.update_edge_flow_y(r_index, commodity.get_demand_vale(), &value);
                        continue;
                    }

                    let d_kp_i: _ = projection_handler.get_d_k_p(&r_index, &value);
                    let lk_p = symmetric_difference(value.clone(), path_s.clone());
                    let h_kp_i = projection_handler.get_h_k_p(&r_index, &lk_p);
                    let x_k_p_t = method_step_commodity.get(&key).or(Some(&0f64)).unwrap();

                    let gh = x_k_p_t - projection_handler.get_alpha() * ((1f64 / h_kp_i) * (d_kp_i - d_kp_s));

                    //вычисляем x_k_p
                    let x_k_p_t_1 = &f64::max(0f64, gh);
                    method_step_commodity.insert(key.to_string(), *x_k_p_t_1); // обновляем поток для следующего шага
                    result += x_k_p_t_1;
                }
                let x_k_p_s_t = commodity.get_demand_vale() - result; // рассчитываем поток для кратчайшего пути
                method_step_commodity.insert(key_path_s.clone(), x_k_p_s_t); // сохраняем результат

                // обновляем ребра графа, для расчета нового кратчайшего маршрута
                if x_k_p_s_t > 0f64 {
                    projection_handler.update_edge_flow(&method_step_commodity, r_index, graph_adj, active_paths_commodity);
                    paths_shortest.insert(r_index, path_s);
                } else {
                    break;
                }
                result_x.insert(r_index, method_step_commodity.clone());
            }
        }
        let delay_value_t_1 = projection_handler.get_delay_value_x();
        let grad_a = Array1::from(grad.to_vec());
        let y_t_a = Array1::from(y_j_t.to_vec());
        let x_t_a = Array1::from(x_j_t.to_vec());
        let diff = x_t_a - y_t_a;
        let t = delay_value_t + grad_a.dot(&diff);
        LB = f64::max(LB, t);

        if delay_value_t_1 <= (1f64 + epsilon) * LB {
            break;
        }
        delay_value_t = delay_value_t_1;

    }

    let duration = start.elapsed();

    for (key, value) in result_x {
        let d = commodities.get(key as usize).unwrap();
        print!("source - {}, target - {} -> paths: ", d.get_source(), d.get_target());
        for (k, v) in value {
            print!("{} - flow: {} ", k, v.to_string());
        }
        println!()
    }


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
