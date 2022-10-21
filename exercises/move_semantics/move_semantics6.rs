// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


fn main() {
    let data = "Rust is great!".to_string();

    // let my_char = get_char(&data);
    // println!("{}", my_char);

    // string_uppercase(data);

    take_ownership_from_immutable(data);

    

    let data2 = "Rust is great!".to_string();
    get_reference_from_immutable(&data2);

    let mut vec0 = Vec::new();
    // vec0.push(22);
    // vec0.push(44);
    // vec0.push(66);

    let vec1 = fill_vec1(vec0);
    // println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    println!("{} has length {} content `{:?}`", "vec0", vec1.len(), vec1);
}

fn fill_vec1(vec: Vec<i32>) -> Vec<i32> {
    // *vec = Vec::new();
    let mut vec = vec;
    vec.push(11);
    vec.push(22);
    vec.push(33);

    vec.to_vec()
}
// Should not take ownership
fn get_char(mut data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}


fn take_ownership_from_immutable(mut data: String) {
    data = data.to_uppercase();
    println!("{}", data);
}


fn get_reference_from_immutable(data: &String) {
    println!("{}", data.chars().last().unwrap());
}