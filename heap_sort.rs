use std::time::Instant;

fn main() {

    let mut array: [i32; 5] = [1, 2, 3, 4, -1];
    let start = Instant::now();

    array = sort(array);    

    let duration = start.elapsed();
    print!("\nSorted array: {:?}\n", array);
    print!("Time elapsed: {:?}.{:?}\n", duration.as_secs(), duration.subsec_nanos());

}

fn sort(mut array: [i32; 5]) -> [i32; 5] {

    print!("Unsorted array {:?}", array);

    let length = array.len();

    if length > 0 {
        heap_sort(&mut array[0..5]);   
    }

    return array;

}

fn heap_sort(array: &mut [i32]) {
    let heap_size = array.len();
    for i in (0..heap_size/2).rev() {
        heapify(array, heap_size, i);
    }

    for i in (1..heap_size).rev() {
        let temp = array[0];
        array[0] = array[i];
        array[i] = temp;

        heapify(array, i, 0)
    }
}

fn heapify(array: &mut [i32], heap_size: usize, node_num: usize) {
    let mut largest = node_num;
    let left_node = 2*node_num+1;
    let right_node = 2*node_num+2;

    if left_node<heap_size && array[largest]<array[left_node] {
        largest = left_node;
    }

    if right_node<heap_size && array[largest]<array[right_node] {
        largest = right_node;
    }

    if largest != node_num {
        let temp = array[largest];
        array[largest] = array[node_num];
        array[node_num] = temp;

        heapify(array, heap_size, largest);
    }
}