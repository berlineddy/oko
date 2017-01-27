
extern crate cheddar;


fn main() {
    cheddar::Cheddar::new()
        .expect("could not read manifest")
        .module("extern_c")
        .expect("malformed module path")
        .run_build("liboko.h");
}
