use std::time::Instant;
use std::vec::Vec;
use std::convert::TryInto;

fn main() {

    let mut array: [i32; 5] = [-211, -311, 100, 3, 100];
    let start = Instant::now();

    array = sort(array);    

    let duration = start.elapsed();
    print!("\nSorted array: {:?}\n", array);
    print!("Time elapsed: {:?}.{:?}\n", duration.as_secs(), duration.subsec_nanos());

}

fn sort(array: [i32; 5]) -> [i32; 5] {

    let mut collected_iterator: Vec<i32> = Vec::from(&array[0..5]);

    print!("Unsorted array {:?}", collected_iterator);

    let length = collected_iterator.len();

    if length > 0 {
        quick_sort(&mut collected_iterator[0..5], 0, length-1);   
    }

    return collected_iterator.try_into().expect("slice with incorrect length");

}

fn quick_sort(array: &mut [i32], start: usize, end: usize) {

    print!("\n\nPass: {:?}, start: {}, end: {}", array, start, end);

    if start<end {
        
        let partition_index = partition(array, start, end);

        print!("\nPartition index: {}, array: {:?}", partition_index, array);

        if end-start>=2 {

            if partition_index != 0 {
                quick_sort(array, start, partition_index-1);
            }

            if partition_index != end {
                quick_sort(array, partition_index+1, end);
            }
        }
    }

}

fn partition(array: &mut [i32], start: usize, end: usize) -> usize {
    let pivot = array[end];

    let mut small_index = start;

    for i in start..end {
        if array[i]<pivot {
            let temp = array[i];
            array[i] = array[small_index];
            array[small_index] = temp;
            small_index += 1;
        }
    }

    let temp = array[small_index];
    array[small_index] = array[end];
    array[end] = temp;

    return small_index;
}