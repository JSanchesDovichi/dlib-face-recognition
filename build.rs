fn main() {
    let mut config = cpp_build::Config::new();

    println!("cargo:rustc-link-lib=openblas");
    println!("cargo:rustc-link-lib=dlib");
    println!("cargo:rustc-link-lib=lapack");

    if let Ok(paths) = std::env::var("DEP_DLIB_INCLUDE") {
        for path in std::env::split_paths(&paths) {
            config.include(path);
        }
    }
    config.flag("-std=c++14").build("src/lib.rs");
}
