fn main() {
    println!("cargo:rustc-link-lib=static=ispack3");
    println!("cargo:rustc-link-lib=static=gfortran");
    println!("cargo:rustc-link-lib=static=quadmath");
    println!("cargo:rustc-link-lib=static=gomp");
}
