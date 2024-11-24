use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn test_algorithm(sort: fn(&mut Vec<i32>), n: usize) {
    let mut array = generate_random_array(n);

    sort(&mut array);
}

fn generate_random_array(n: usize) -> Vec<i32> {
    let mut array: Vec<i32> = (0..(n as i32)).collect();

    array.shuffle(&mut thread_rng());

    array
}
