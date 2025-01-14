fn average(values: &[f64]) -> f64 {
    let total = values
        .iter()//是运用了一个迭代器吗
        .fold(0.0, |a, b| a + b);
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}
