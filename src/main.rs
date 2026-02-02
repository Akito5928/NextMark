fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: nextmark <input.md>");
        return;
    }

    let input = std::fs::read_to_string(&args[1])
        .expect("failed to read input file");

    let html = nextmark::render(&input);

    println!("{}", html);
}
