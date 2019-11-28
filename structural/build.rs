use rustc_version::Version;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let rver = rustc_version::version().unwrap();

    if Version::new(1, 36, 0) <= rver {
        println!("cargo:rustc-cfg=feature=\"rust_1_36\"");
    }
    if Version::new(1, 40, 0) <= rver {
        println!("cargo:rustc-cfg=feature=\"rust_1_40\"");
        println!("cargo:rustc-cfg=feature=\"better_macros\"");
    }
}
