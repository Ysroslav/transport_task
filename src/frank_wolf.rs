use ndarray::{arr1, Array, array, Array1, Array2, Axis};

/// рачет функции скалярной |Ax - y|^2
/// a - матрица
/// x - вектор, стобец с координатам
/// y - вектор
pub fn count_norma(a: &Array2<f64>, x: &Vec<f64>, y: &Vec<f64>) -> f64 {
    let x_vec = arr1(x);
    let y_vec = arr1(y);
    let binding = (a.dot(&x_vec) - y_vec);
    let b = binding.view();
    b.dot(&b).powi(2)
}

/// расчет градиента по формуле 2 * (A^T(Ax - y))
/// a - матрица
/// x - вектор, стобец с координатам
/// y - вектор
/// функция возвращат вектор
pub fn count_gradient(a: &Array2<f64>, x: &Vec<f64>, y: &Vec<f64>) -> Vec<f64> {
    let x_vec = arr1(x);
    let y_vec = arr1(y);
    let binding = &(a.dot(&x_vec) - y_vec);
    let mut a_m = a.view();
    let a_transport = a_m.reversed_axes();
    let b = a_transport.dot(binding) * 2.0;
    b.to_vec()
}

/// реализация оракула для реализаци алгоритма
/// a - матрица
/// x - вектор, стобец с координатам
/// y - вектор
/// функция возвращат вектор
pub fn create_oracle(size: f64, req_param: f64, a: &Array2<f64>, x: &Vec<f64>, y: &Vec<f64>) -> Vec<f64> {

    let mut s = vec![0.0, size];
    let grad = count_gradient(a, x, y);
    if grad.iter().all(|&x| x == 0.0) {
        return s;
    }
    let i = grad.iter().map(|x| x.abs()).reduce(f64::max).unwrap() as usize;
    s[i] = - grad[i].signum() * req_param;
    return s
}

///реалзация алгоритма Франк-Вульф
pub fn frank_wolfe(a: &Array2<f64>, y: &Vec<f64>, k: i32) -> Vec<f64> {

    /// количество итераций
    const COUNT_ITERATION: usize = 2000;

    /// параметр регуляции
    const req_param: f64 = 164f64;

    /// допустимая точность
    const epsilon: f64 = 0.001;


    let size = a.len_of(Axis(0));

    let mut x = Array2::<f64>::zeros((size, COUNT_ITERATION));
    let mut s = Array2::<f64>::zeros((size, COUNT_ITERATION));

    /// определение шага спуска
    let mut arr_step = [f64::NAN; COUNT_ITERATION];

    /// массив записи результата
    let mut result = [f64::NAN; COUNT_ITERATION];

    /// массив разности f(x(n)) - f(x(n-1))
    let mut arr_difference = [f64::NAN; COUNT_ITERATION];

    for i in 1..COUNT_ITERATION {
        arr_step[i] = f64::from(2) / (f64::from(2) + f64::from(k));
        let g = count_gradient(a, &x.row(i - 1).to_vec(), y);
        s.push_row(Array1::from(create_oracle(size as f64, req_param, a, &g, y)).view()).expect("TODO: panic message");
        let x1 = Array::from(x.row(i - 1).to_vec()) * (f64::from(1) - arr_difference[i]);
        let x2 = Array::from(s.row(i).to_vec()) * arr_step[i];
        x.push_row((x1 + x2).view()).expect("TODO: panic message");
        result[i] = count_norma(a, &x.row(i - 1).to_vec(), y);
        if i > 1 {
            arr_difference[i - 1] = result[i] - result[i - 1];
            if epsilon >= arr_difference[i - 1].abs() {
                break
            }
        }
    }
    result.to_vec()
}

