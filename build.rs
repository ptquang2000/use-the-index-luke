fn main() {
    println!(
        "cargo::rustc-link-search=native={}/build",
        env!("CARGO_MANIFEST_DIR")
    );
}
