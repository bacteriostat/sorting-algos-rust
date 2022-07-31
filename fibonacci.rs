fn main(){

    let mut limit: u32 = 10;

    let mut first: u32 = 0;
    let mut second: u32 = 1;

    print!("{}\n", first);
    limit = limit - 1;
    print!("{}\n", second);
    limit = limit - 1;

    loop {
        let sum: u32 = first+second;
        print!("{}\n", sum);
        first=second;
        second=sum;

        limit = limit - 1;
        
        if limit == 0 {
            break;
        }
    }
}