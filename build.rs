fn main() {
    if std::env::var("CARGO_FEATURE_FIRST_BIN").is_ok() {
        println!("cargo:warning=BIN_NAME: first_bin");
    } else if std::env::var("CARGO_FEATURE_SECOND_BIN").is_ok() {
        println!("cargo:warning=BIN_NAME: second_bin");
    }
}