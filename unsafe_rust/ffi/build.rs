fn main() {
    cc::Build::new().file("src/mylib.c").compile("my_c_lib.a");
}
