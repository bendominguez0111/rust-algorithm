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

pub fn standard_deviation()
{}