//utility functions for doing math

pub fn simple_moving_average(data: &[f64], window_size: usize) -> f64
{
    let data_len = data.len();
    if window_size > data_len {
        let window_size = data_len; // shadowing
        let window_sum: f64 = data.iter().sum();
        return window_sum / window_size as f64
    }
    else {
        let window_sum: f64 = data[data_len - window_size..].iter().sum();
        return window_sum / window_size as f64
    }
}

pub fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();

    sum / (data.len() as f64)
}

pub fn std_dev(data: &[f64]) -> f64 {
    let mu = mean (&data);
    let sum_squared_diffs: f64 = data
        .iter()
        .map(|&value| (value - mu).powi(2))
        .sum();
    let var = sum_squared_diffs / (data.len() as f64);

    var.sqrt()
}

pub fn pct_change(data: &[f64]) -> Vec<f64> {
    data
        .windows(2)
        .map(|window| (window[1] - window[0]) / window[0] * 100.)
        .collect()
}