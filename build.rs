static TARGET_PATH: &str = "../CirnoUser/target/riscv64gc-unknown-none-elf/release/";

fn main() {
    println!("cargo:rerun-if-changed=../CirnoUser/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
}
