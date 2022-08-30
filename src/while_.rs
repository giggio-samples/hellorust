pub fn the_while(i: u32) {
    print!("While ");
    let mut j = 0;
    while j < i {
        print!("{i} ");
        j += 1;
    }
    println!();
}
