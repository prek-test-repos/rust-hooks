use itoa::Buffer;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        // Use itoa to format a number, proving the dependency works
        let mut buffer = Buffer::new();
        let s = buffer.format(42u32);
        println!("{s}");
    } else {
        println!("{}", args.join(" "));
    }
}
