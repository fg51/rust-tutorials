fn main() {
    println!("Hello, world!");
    sample_range_next();
    sample_foreach();
    sample_collect();
    sample_find();
    sample_fold();
    sample_map();
    sample_take();
    sample_filter();
}

fn sample_range_next() {
    println!("sample range next");
    let mut range = 0..10;
    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            }
            None => break,
        }
    }
}

fn sample_foreach() {
    println!("sample foreach");
    let xs = vec![1, 2, 3];
    for x in &xs {
        println!("{}", x);
    }
}

fn sample_collect() {
    println!("sample collect");
    let one_to_five = (1..6).collect::<Vec<_>>();
    for i in &one_to_five {
        println!("{}", i);
    }
}

fn sample_find() {
    println!("sample find");
    let greater_than_two = (0..8).find(|&x| x > 42);
    match greater_than_two {
        Some(_) => println!("Found a match"),
        None => println!("No match found"),
    }
}

fn sample_fold() {
    println!("sample fold");
    let sum = (1..4).fold(0, |sum, x| sum + x);
    println!("sum: {0}", sum);
}

fn sample_map() {
    println!("sample map");
    for i in (1..5).map(|x| x * 10) {
        println!("{}", i);
    }
}

fn sample_take() {
    println!("sample take");
    for i in (1..).take(5) {
        println!("{}", i);
    }
}

fn sample_filter() {
    println!("sample filter");
    for i in (1..10).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }
}
