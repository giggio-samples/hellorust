pub fn expr_if(cond: bool) -> i32 {
    println!("Expression If");
    let x = if cond {
        println!("dentr do if {}", cond);
        1
    } else {
        2
    };
    x + 1
}

pub fn expr_match(i: u32) -> i32 {
    println!("Expression Match");
    let x = match i {
        1 => 1,
        2 => 2,
        _ => panic!(),
    };
    x + 1
}
