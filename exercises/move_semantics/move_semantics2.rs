// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // vec1.push(88);

    // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    // let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}

// Original code

// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

////////////////////
// First way, pass a new variable
//
// fn main() {
//     let vec0:Vec<i32> = Vec::new();
//     let vec01 = vec![0];
//     let mut vec1 = fill_vec(vec01);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

////////////////////
// Second way, return a new variable
//
// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(&vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec01 :Vec<i32> = Vec::new();

//     vec01.push(22);
//     vec01.push(44);
//     vec01.push(66);

//     vec01
// }

////////////////////
// Third way, change the original and return it
// fn main() {
//     let mut vec0 = Vec::new();

//     fill_vec(&mut vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     // vec1.push(88);

//     // println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
//     // let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec.to_vec()
// }