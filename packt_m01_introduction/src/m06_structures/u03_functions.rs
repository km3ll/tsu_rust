//! # Functions within a Trait

#[derive(Debug)]
struct Data {
    sample: Vec<i32>,
}

trait Stats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl Stats for Data {
    fn mean(&self) -> f32 {
        let mut sum: i32 = 0;
        for e in self.sample.iter() {
            sum += e;
        }
        sum as f32 / self.sample.len() as f32
    }

    fn variance(&self) -> f32 {
        let mean = self.mean();
        let mut sum_squared_diff: f32 = 0.0;
        for e in self.sample.iter() {
            sum_squared_diff += (*e as f32 - mean) * (*e as f32 - mean);
        }
        sum_squared_diff / self.sample.len() as f32
    }
}

fn functions_in_traits() {
    let n1 = r#"
    pod: Functions in Traits
    - Can call other functions within the same trait
    ---"#;
    println!("{n1}");

    println!("Functions in Traits");
    let data = Data {
        sample: vec![190, 200, 210],
    };
    println!(" > data: {data:?}");
    println!(" > mean: {}", data.mean());
    println!(" > variance: {}", data.variance());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_functions_in_traits() {
        functions_in_traits()
    }
}
