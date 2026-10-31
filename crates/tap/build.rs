//! Links `tap.def` so `DllGetClassObject` is exported PRIVATE (LNK4104).

fn main() {
    println!("cargo:rerun-if-changed=tap.def");
    let def =
        std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("tap.def");
    println!("cargo:rustc-link-arg=/DEF:{}", def.display());
}
