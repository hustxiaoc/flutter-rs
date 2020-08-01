use flutter_glfw::{init, window::WindowArgs, window::WindowMode};
use std::path::{Path, PathBuf};

fn main() {
    env_logger::init();

    for argument in std::env::args() {
        println!("*** args = {}", argument);
    }

    let p = dirs::cache_dir()
            .expect("Cannot get cache dir")
            .join("flutter-engine");

    println!("dir is {:?}", p);

    let assets_dir = std::env::var("FLUTTER_ASSET_DIR").expect("FLUTTER_ASSET_DIR");
    println!("assets_dir is {:?}", assets_dir);

    let mut args = Vec::with_capacity(3);

    if let Ok(observatory_port) = std::env::var("DART_OBSERVATORY_PORT") {
        args.push("--disable-service-auth-codes".to_string());
        args.push(format!("--observatory-port={}", observatory_port));
    }

    if let Ok(snapshot) = std::env::var("FLUTTER_AOT_SNAPSHOT") {
        if Path::new(&snapshot).exists() {
            args.push(format!("--aot-shared-library-name={}", snapshot));
        }
    }

    println!("create flutter window, running flutter app with args {:?}", args);

    let mut flutter_desktop = init().unwrap();
    let window_args = WindowArgs{
        width: 800,
        height: 500,
        title: "flutter",
        mode: WindowMode::Windowed,
    };

    println!("create flutter_desktop");

    let flutter = flutter_desktop.create_window(&window_args, PathBuf::from(assets_dir), args).unwrap();

    println!("start run");
    flutter.run(None, None).unwrap();
    println!("done");
}
