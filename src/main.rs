fn main() {
    let crate_version: &'static str = env!("CARGO_PKG_VERSION");
    print!("Brainfuck Interpreter v{}", crate_version);
}
