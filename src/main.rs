mod for_;
mod while_;
use for_::the_for;
mod expressions;
mod lifetimes;
mod patterns;
mod structs;
mod test;

fn main() {
    println!("Hello, world!");
    the_for(5);
    while_::the_while(5);
    expressions::expr_if(true);
    expressions::expr_match(2);
    println!("Sum is: {}", test::sum(1, 2));
    patterns::pattern_match(2);
    patterns::if_let(Some(3));
    patterns::if_let(None);
    structs::use_structs();
    lifetimes::use_lifetimes();
}
