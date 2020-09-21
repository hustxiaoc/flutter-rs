fn main() {
    let p = "/Users/taoxiaojie/work/flutter_demo/rust-flutter/examples/flutter-unit/target/flutter/debug";
    // println!("rustc-flags='-C link-arg=-Wl,-rpath=@loader_path/.'");
    println!("cargo:rustc-link-search=framework={}", p);
    println!("cargo:rustc-link-lib=framework=FlutterEmbedder");
}
