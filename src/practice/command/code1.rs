use std::process::Command;

fn main() {
    let output = Command::new("C:\\Windows\\System32\\cmd.exe")
        // /c means that carry out the command and then terminate
        // /k === !/c means that carry out the command and then keep the window open
        // .args(&["dir"]) <- /k or not use /c
        .args(&["/c", "dir"])
        .output()
        .expect("Failed to execute command");

    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
}
