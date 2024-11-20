pub fn count_delay_value(x: f32, c: f32) -> f32 {
    if c == x {
        return 0f32;
    }
    return x / (c - x)
}

pub fn count_first_derivative(x: f64, c: f64) -> f64 {
    /*if c == x {
        return 0f64;
    }*/
    return c / (c - x)//.powi(2)
}

pub fn count_second_derivative(x: f64, c: f64) -> f64 {
    if c == x {
        return 0f64;
    }
    return (2f64 * c) / (c - x).powi(3)
}