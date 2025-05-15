fn main() {
    // inject emscripten build options
    if std::env::var("TARGET").is_ok_and(|v| v.contains("emscripten")) {
        println!("cargo::rustc-link-arg=-sINVOKE_RUN=0");
        println!("cargo::rustc-link-arg=-sEXPORTED_FUNCTIONS=_init_audio,_shutdown_audio");
        println!("cargo::rustc-link-arg=-sEXPORTED_RUNTIME_METHODS=ccall,cwrap");
        println!("cargo::rustc-link-arg=--no-entry");
    }
}
