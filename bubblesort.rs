use std::time::Instant;

fn main() {

    let mut array: [i32; 5] = [2,3,4,5,1];

    let start = Instant::now();

    array = sort(array);    

    let duration = start.elapsed();

    print!("\nSorted array: {:?}\n", array);

    print!("Time elapsed: {:?}.{:?}\n", duration.as_secs(), duration.subsec_nanos());

}

fn sort(mut array: [i32; 5]) -> [i32; 5] {
    let mut last_swap_position;

    let mut i = 0;

    while i != array.len() - 1 {
        last_swap_position = 0;
        for n in 0..array.len()-1 {
            if array[n]>array[n+1] {
                let temp: i32 = array[n];
                array[n] = array[n+1];
                array[n+1] = temp;
                last_swap_position=n+1;
            }
        }

        print!("Last swap position: {}\n", last_swap_position);

        print!("Pass {}: {:?}\n", i, array);

        i = array.len() - 1 - last_swap_position;
    }

    return array
}