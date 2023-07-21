use std::process::Command;
fn main() {
    let status = Command::new("LemnisGate/Binaries/Win64/LemnisGate-Win64-Shipping.exe").status().expect("failed to execute process");
    println!("process finished with: {status}");
    assert!(status.success());
}
