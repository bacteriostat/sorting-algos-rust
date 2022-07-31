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
        let mut min_pos = i;
        for j in i+1..array.len(){
            if array[j]<array[min_pos] {
                min_pos = j
            }
        }
        let temp = array[min_pos];
        array[min_pos] = array[i];
        array[i] = temp;
        print!("Pass {}: {:?}\n", i, array);
    }

    return array
}