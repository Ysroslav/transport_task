pub fn count_delay_value(x: f64, c: f64, p: f64) -> f64 {
    if x > p * c {
        let k = count_first_derivative(x,c,p);
        let m = count_second_derivative(p * c, c, p);
        return p * c / (c - p * c) + k * (x - p*c) + m / 2f64 * (x - p*c).powi(2);
    }
    return x / (c - x)
}

/*
если x больше чем pc используем линейную функцию,
находим производную в точке pc, так  как функция линейная, и производная везде постоянна,
то она будет равна производной в точке x
 */
pub fn count_first_derivative(x: f64, c: f64, p: f64) -> f64 {
    if x > p * c {
        let k = count_second_derivative(p * c, c, p);
        return k * (x - p*c) + c / (c - p * c).powi(2);
    }
    return c / (c - x).powi(2)
}

/*
если x больше чем pc используем линейную функцию,
вторая производная линейной функции равна 0
 */
pub fn count_second_derivative(x: f64, c: f64, p: f64) -> f64 {
    if x > p * c {
        return (2f64 * c) / (c - p * c).powi(3);
    }
    return (2f64 * c) / (c - x).powi(3)
}