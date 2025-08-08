fn main() {
    // This tells Cargo to rerun the build script if device.yaml changes
    println!("cargo:rerun-if-changed=device.yaml");
}
