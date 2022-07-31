use std::time::Instant;
use std::vec::Vec;
use std::convert::TryInto;

fn main() {

    let mut array: [i32; 5] = [100, -101, -100, 102, 101];
    let start = Instant::now();

    array = sort(array);    

    let duration = start.elapsed();
    print!("\nSorted array: {:?}\n", array);
    print!("Time elapsed: {:?}.{:?}\n", duration.as_secs(), duration.subsec_nanos());

}

fn sort(array: [i32; 5]) -> [i32; 5] {

    let mut collected_iterator: Vec<i32> = Vec::from(&array[0..5]);

    print!("Unsorted array {:?}", collected_iterator);

    if collected_iterator.len() > 0 {
        collected_iterator = merge_sort(collected_iterator);   
    }

    return collected_iterator.try_into().expect("slice with incorrect length");
    // return array;

}

fn merge_sort(array: Vec<i32>) -> Vec<i32> {

    if array.len() == 1 {
        return array;
    }

    let left = 0;
    let right = array.len();
    
    let mut left_part: Vec<i32> = Vec::from(&array[left..(right/2)]);
    let mut right_part: Vec<i32> = Vec::from(&array[(right/2)..right]);

    left_part = merge_sort(left_part);
    right_part = merge_sort(right_part);

    return merge(left_part, right_part);

}

fn merge(left_part: Vec<i32>, right_part: Vec<i32>) -> Vec<i32> {

    let mut combined: Vec<i32> = Vec::new();

    let mut i = 0; 
    let mut j = 0;

    while i < left_part.len() && j < right_part.len() {
        if left_part[i] < right_part[j] {
            combined.push(left_part[i]);
            i = i + 1;
        }else{
            combined.push(right_part[j]);
            j = j + 1;
        }
    }

    while i < left_part.len() {
        combined.push(left_part[i]);
        i = i + 1;
    }

    while j < right_part.len() {
        combined.push(right_part[j]);
        j = j + 1;
    }

    return combined;
}