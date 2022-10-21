// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.


fn main() {
    let (_a, _v) = array_and_vec();
}

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![10, 20, 30, 40];

    let my_arr = [11, 22, 33, 44];
    let my_vec = vec![11, 22, 33, 44];
    println!("{:?}", my_arr);
    println!("{:?}", my_vec);

    if my_arr == my_vec[..] {
        println!("Same!")
    } else {
        println!("Not same!")
    }

    return (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
