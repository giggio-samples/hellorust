pub fn pattern_match(i: u32) {
    println!("Match");
    match i {
        1 => println!("1"),
        2 => println!("2"),
        _ => panic!(),
    };
}

pub fn if_let(x: Option<u32>) {
    print!("If let: ");
    if let Some(i) = x {
        println!("{}", i);
    } else {
        println!("None");
    }
}
