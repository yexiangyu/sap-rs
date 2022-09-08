fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/sap.cpp")
        .flag_if_supported("-std=c++14")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/blobstore.cc");
    println!("cargo:rerun-if-changed=include/blobstore.h");
}
