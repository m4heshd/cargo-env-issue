fn main() {
    if let Ok(bin_name) = std::env::var("BIN_NAME") {
        println!("cargo:warning=BIN_NAME: {:?}", bin_name);
    }
}