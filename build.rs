fn main() {
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/fmod-api/");
        println!("cargo:rustc-env=LD_LIBRARY_PATH=/usr/lib/x86_64-linux-gnu/fmod-api/");
    }
}
