use std::{
    env,
    fs::{self, File},
    process::Command,
};

const CARGO_BUILD_RUSTFLAGS: &str = concat!(
    r#" --remap-path-prefix=$PWD=_"#,
    r#" --remap-path-prefix=$HOME/.local/lib/cargo=-"#,
    r#" --remap-path-prefix=$HOME/.local/lib/rustup=+"#,
    r#" --remap-path-prefix=$HOME=~"#,
    r#" --remap-path-prefix=$HOME/.cargo/registry/src/=%"#,
    r#" --cfg"#,
    r#" target_os="daku""#,
    r#" -C"#,
    r#" link-args=-zstack-size=32768"#,
);

fn main() {
    println!("Creating local target directory…");

    fs::create_dir_all("./target").expect("Failed to create target dir");

    println!("Compiling…");

    let current_dir = env::current_dir().expect("Couldn't get current dir");
    let target = current_dir.join("target");
    let path = {
        let mut path = env::var_os("PATH").expect("No path!");
        path.push(":");
        path.push(current_dir);
        path.push("/target/bin");

        path
    };

    Command::new("cargo")
        .arg("install")
        .arg("--path")
        .arg(".")
        .arg("--root")
        .arg(target)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_BUILD_RUSTFLAGS", CARGO_BUILD_RUSTFLAGS)
        .env("PATH", path)
        .spawn()
        .expect("failed to execute cargo")
        .wait()
        .expect("failed to wait for cargo");

    println!("Snipping panicking code…");

    Command::new("wasm-snip")
        .arg("./target/bin/hello.wasm")
        .arg("--snip-rust-panicking-code")
        .arg("-o")
        .arg("./target/bin/hello.wasm")
        .arg("--")
        .arg("main")
        .spawn()
        .expect("failed to execute cargo")
        .wait()
        .expect("failed to wait for cargo");

    println!("Optimizing…");

    Command::new("wasm-opt")
        .arg("./target/bin/hello.wasm")
        .arg("-o")
        .arg("./target/bin/hello.wasm")
        .arg("-Os")
        .spawn()
        .expect("failed to execute cargo")
        .wait()
        .expect("failed to wait for cargo");

    println!("Stripping…");

    Command::new("wasm-strip")
        .arg("./target/bin/hello.wasm")
        .arg("-k")
        .arg("producers")
        .spawn()
        .expect("failed to execute cargo")
        .wait()
        .expect("failed to wait for cargo");

    let bytes = File::open("./target/bin/hello.wasm")
        .expect("Failed to open file")
        .metadata()
        .expect("Failed to get file metadata")
        .len();

    println!("Done! ({bytes} bytes)");
}
