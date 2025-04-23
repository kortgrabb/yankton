use arboard::{Clipboard, LinuxClipboardKind, SetExtLinux};
use std::io::{self, Write};

fn main() {
    let mut ctx = Clipboard::new().unwrap();
    let mut input = String::new();
    println!("Enter text to copy to clipboard:");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed before input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim(); // Remove any trailing newline characters
    if input.is_empty() {
        println!("No text entered. Exiting.");
        return;
    }
    // TODO: Add error handling for clipboard operations
    // Set the text on the primary clipboard and make it persist after the program exits.
    ctx.set()
        .clipboard(LinuxClipboardKind::Clipboard)
        .wait() // This makes the program wait until another application requests the clipboard content.
        .text(input)
        .unwrap();
    println!("Text copied to clipboard: {}", input);
}
