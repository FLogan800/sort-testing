// Bubble Sort
// Runs in O(n^2) time

pub fn sort<T: Ord>(array: &mut Vec<T>) {
    let mut n = array.len();

    loop {
        let mut swapped = false;

        for i in 1..n {
            if array[i] < array[i - 1] {
                array.swap(i, i - 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        n -= 1;
    }
}
