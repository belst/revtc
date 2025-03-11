use revtc::open_raw;

fn main() {
    let mut args = std::env::args();
    let _program_name = args.next().unwrap();
    let filename = args.next().expect("No filename given");
    let file = open_raw(filename).expect("Could not open file");
    println!("{file:#?}");
}
