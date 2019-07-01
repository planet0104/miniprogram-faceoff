fn main() {
    cc::Build::new()
        .file("src\\library.c")
        .file("pico/rnt/picornt.c")
        .include("pico/rnt/")
        .compile("library");
}
