use archforge::parser::parse_program;

fn main() {
    let p = parse_program("increment(x) {
        ADD $counter, %x, $counter
    }
    
    main() {
        increment(5)
    }").unwrap();

    println!("{p:?}");
}