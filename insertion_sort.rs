use std::time::Instant;

fn main() {

    let mut array: [i32; 5] = [90,8,2,-3,-9];
    let start = Instant::now();

    array = sort(array);    

    let duration = start.elapsed();
    print!("\nSorted array: {:?}\n", array);
    print!("Time elapsed: {:?}.{:?}\n", duration.as_secs(), duration.subsec_nanos());

}

fn sort(mut array: [i32; 5]) -> [i32; 5] {
    print!("Unsorted array: {:?}\n\n", array);
    for i in 0..array.len() {
        for j in (1..i+1).rev() {
            if array[j]<array[j-1] {
                let temp = array[j];
                array[j] = array[j-1];
                array[j-1] = temp;
            }
        }
        print!("Pass {}: {:?}\n", i, array);
    }

    return array
}