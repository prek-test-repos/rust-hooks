fn main() {
    for arg in std::env::args().skip(1) {
        print!("{}", arg);
    }
    print!("\n");
}
