//! # Iterators - Part 2

fn iterators() {
    let n1 = r#"
    ---
    pod: Iterators
    - The collect function transforms an iterator into a collection (arrays, vectors)
    ---"#;
    println!("{n1}");
}

fn iterators_collect() {
    println!("Iterators");

    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!(" > vec1: {vec1:?}");

    let res1: Vec<&i32> = vec1.iter().filter(|&x| *x % 2 > 0).collect::<Vec<&i32>>();
    println!(" > filter(is_odd).collect() Vec<&i32> res1: {res1:?}");
}

fn iterator_into_iter() {
    println!("Iterators");

    let vec2: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!(" > vec2: {vec2:?}");

    println!(" > consuming iterator");
    let res2: Vec<i32> = vec2
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect::<Vec<i32>>();
    println!(" > into_iter().collect Vec<i32> res2: {res2:?}");
}

fn iterator_clone() {
    println!("Iterators");

    let vec3: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > vec3: {vec3:?}");

    let vec4: Vec<i32> = vec3.clone();
    println!(" > cloned vec4: {vec4:?}");
}

fn iterator_map() {
    println!("Iterators");

    let vec5: Vec<i32> = vec![10, 15, 40, 45, 30];
    println!(" > vec5: {vec5:?}");

    let vec6 = vec5.iter().map(|x| x * 11).collect::<Vec<i32>>();
    println!(" > map(x2) vec6: {vec6:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_iterators() {
        iterators()
    }

    #[test]
    fn run_iterators_collect() {
        iterators_collect()
    }

    #[test]
    fn run_iterator_into_iter() {
        iterator_into_iter()
    }

    #[test]
    fn run_iterator_clone() {
        iterator_clone()
    }

    #[test]
    fn run_iterator_map() {
        iterator_map()
    }
}
