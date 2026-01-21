use std::ops::Range;
use std::slice::Iter;

fn vectors() {
    let n1 = r#"
    pod: Vector
    - A collection of similar elements that can be resized
    - Elements are store in contiguous memory locations
    ---"#;
    println!("{n1}");

    let v1: Vec<i32> = vec![10, 20, 30, 40, 50];
    println!("Vectors: v1: {:?}", v1);
}

fn vectors_initialize() {
    let v2: Vec<bool> = vec![true; 3];
    println!("Vectors: initialize: v2: {:?}", v2);
}

fn vectors_push() {
    println!("Vectors: push");

    let mut v3: Vec<i32> = vec![1, 2, 3];
    println!(" > before: v3: {:?}", v3);

    v3.push(4);
    println!(" > after : v3: {:?}", v3);
}

fn vectors_pop() {
    println!("Vectors: pop");

    let mut v4: Vec<i32> = vec![1, 2, 3];
    println!(" > before: v4: {:?}", v4);

    let e4: Option<i32> = v4.pop();
    println!(" > after : v4: {:?}, e4: {:?}", v4, e4);
}

fn vectors_remove() {
    println!("Vectors: remove");

    let mut v5: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > before: v5: {:?}", v5);

    v5.remove(2);
    println!(" > after : v5: {:?}", v5);
}

fn vectors_sort() {
    println!("Vectors: sort");

    let mut v6 = vec![5, 3, 9, 8];
    println!(" > before: v6: {:?}", v6);

    v6.sort();
    println!(" > after : v6: {:?}", v6);
}

fn vectors_reverse() {
    println!("Vectors: reverse");

    let mut v7 = vec![1, 2, 3, 4, 5];
    println!(" > before: v7: {:?}", v7);

    v7.reverse();
    println!(" > after : v7: {:?}", v7);
}

fn vectors_slices() {
    println!("Vectors: slices");

    let v8: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > v8: {:?}", v8);

    let s1: &[i32] = &v8[1..3];
    println!(" > start-inclusive, end-exclusive [1..3]: {:?}", s1);

    let s2: &[i32] = &v8[1..=3];
    println!(" > start-inclusive, end-inclusive [1..=3]: {:?}", s2);

    let s3: &[i32] = &v8[..3];
    println!(" > from beginning to index 3 exclusive [..3]: {:?}", s3);

    let s4 = &v8[..];
    println!(" > entire collection [..]: {:?}", s4);
}

fn vectors_iter() {
    println!("Vectors: iter");

    let v9: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > v9: {:?}", v9);

    let i9: Iter<i32> = v9.iter();
    for e in i9 {
        println!(" > e: {e}");
    }
}

fn vectors_update() {
    let n1 = r#"
    pod: Dereference Operator (*)
    - *e dereferences first, then modify
    - Without *, e is just a reference (pointer)
    - With *, you get the actual value that the reference points to
    ---"#;
    println!("{n1}");

    println!("Vectors: update");

    let mut v10: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > before: v10: {:?}", v10);

    for e in &mut v10 {
        *e += 10;
    }
    println!(" > after: v10: {:?}", v10);
}

fn vectors_capacity() {
    println!("Vectors: capacity");

    let v11 = vec![0, 1, 2, 3, 4, 5];
    println!(" > v1: {:?}", v11);

    let u1 = v11.capacity();
    println!(" > u1: {u1}");
}

fn vectors_resize() {
    println!("Vectors: resize");

    let mut v12 = vec![1, 2, 3, 4, 5];
    println!(" > before v12: {:?}", v12);

    v12.resize(3, 0);
    println!(" > after v12: {:?}", v12);

    v12.resize(6, 0);
    println!(" > after v12: {:?}", v12);
}

fn vectors_len() {
    println!("Vectors: len");

    let v13 = vec![1, 2, 3, 4, 5];
    println!(" > v13: {:?}", v13);

    let u1: usize = v13.len();
    println!(" > u1: {u1}");
}

fn vectors_is_empty() {
    println!("Vectors: is_empty");

    let v14: Vec<u16> = vec![];
    println!(" > v14: {:?}", v14);

    let b1: bool = v14.is_empty();
    println!(" > b1: {b1}");
}

fn vectors_clear() {
    println!("Vectors: clear");

    let mut v15 = vec![1, 2, 3, 4, 5];
    println!(" > before: v15: {:?}", v15);

    v15.clear();
    println!(" > after: v15: {:?}", v15);
}

fn vectors_into_iter() {
    println!("Vectors: into_iter");

    let v16: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > v16: {:?}", v16);

    for e in v16.into_iter() {
        println!(" > e: {e}");
    }
}

fn vectors_from_range() {
    println!("Vectors: from range");

    let r15: Range<i32> = (0..10);
    println!(" > r15: {:?}", r15);

    let v15: Vec<i32> = r15.collect();
    println!(" > v15: {:?}", v15);
}

fn vector_contains() {
    println!("Vectors: contains");

    let v16: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > v16: {:?}", v16);

    let b1: bool = v16.contains(&3);
    println!(" > b1: {b1}");
}

fn vectors_binary_search() {
    println!("Vectors: binary_search");

    let v17: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > v17: {:?}", v17);

    let s1: Result<usize, usize> = v17.binary_search(&3);
    println!(" > s1: {:?}", s1);
}

fn vectors_index() {
    println!("Vectors: index");

    let v18: Vec<char> = vec!['a', 'b', 'c'];
    println!(" > v18: {:?}", v18);

    println!(" > index #1: {:?}", v18[1]);
}

fn vectors_get() {
    println!("Vectors: get");

    let v19: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > v19: {:?}", v19);

    println!(" > get index #4: {:?}", v19.get(4));
}

fn vectors_get_mut() {
    println!("Vectors: get_mut");

    let mut v20: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!(" > before v20: {:?}", v20);

    *v20.get_mut(1).unwrap() = 100;
    println!(" > after v20: {:?}", v20);
}

fn vectors_insert() {
    println!("Vectors: insert");

    let mut v21: Vec<i32> = vec![1, 2, 5, 7];
    println!(" > before v21: {:?}", v21);

    v21.insert(2, 50);
    println!(" > after v21: {:?}", v21);
}

fn vectors_dedup() {
    println!("Vectors: dedup");

    let mut v22: Vec<i32> = vec![1, 2, 2, 3, 3, 3];
    println!(" > before v22: {:?}", v22);

    v22.dedup();
    println!(" > after v22: {:?}", v22);
}

fn vectors_split_at() {
    println!("Vectors: split_at");

    let v23: Vec<&str> = vec!["a", "b", "c", "d", "e"];
    println!(" > v23: {:?}", v23);

    println!(" > split at #3");
    let (l1, r1) = v23.split_at(3);

    println!(" > left: {:?}", l1);
    println!(" > right: {:?}", r1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_vectors() {
        vectors()
    }

    #[test]
    fn run_vectors_initialized() {
        vectors_initialize()
    }

    #[test]
    fn run_vectors_push() {
        vectors_push()
    }

    #[test]
    fn run_vectors_pop() {
        vectors_pop()
    }

    #[test]
    fn run_vectors_remove() {
        vectors_remove()
    }

    #[test]
    fn run_vectors_sort() {
        vectors_sort()
    }

    #[test]
    fn run_vectors_reverse() {
        vectors_reverse()
    }

    #[test]
    fn run_vectors_slices() {
        vectors_slices()
    }

    #[test]
    fn run_vectors_iter() {
        vectors_iter()
    }

    #[test]
    fn run_vectors_update() {
        vectors_update()
    }

    #[test]
    fn run_vectors_capacity() {
        vectors_capacity()
    }

    #[test]
    fn run_vectors_resize() {
        vectors_resize()
    }

    #[test]
    fn run_vectors_len() {
        vectors_len()
    }

    #[test]
    fn run_vectors_is_empty() {
        vectors_is_empty()
    }

    #[test]
    fn run_vectors_clear() {
        vectors_clear()
    }

    #[test]
    fn run_vectors_into_iter() {
        vectors_into_iter()
    }

    #[test]
    fn run_vectors_from_range() {
        vectors_from_range()
    }

    #[test]
    fn run_vector_contains() {
        vector_contains()
    }

    #[test]
    fn run_vectors_binary_search() {
        vectors_binary_search()
    }

    #[test]
    fn run_vectors_index() {
        vectors_index()
    }

    #[test]
    fn run_vectors_get() {
        vectors_get()
    }

    #[test]
    fn run_vectors_get_mut() {
        vectors_get_mut()
    }

    #[test]
    fn run_vectors_insert() {
        vectors_insert()
    }

    #[test]
    fn run_vectors_dedup() {
        vectors_dedup()
    }

    #[test]
    fn run_vectors_split_at() {
        vectors_split_at()
    }
}
