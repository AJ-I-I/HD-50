mod crypto;
mod vault;

use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, 
    terminal::{Clear, ClearType, SetTitle},
    style::{Color, SetForegroundColor, ResetColor, Print}
};

fn main() {
    setup_terminal();
    
    print_banner();
    // Check if UNLOCKED_CONTENTS exists
    
    if std::path::Path::new("UNLOCKED_CONTENTS").exists() {
        println!("Detected UNLOCKED contents.");
        print!("Do you want to LOCK and SECURE them now? [Y/n]: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().eq_ignore_ascii_case("n") {
             println!("Exiting. WARNING: Files are UNPROTECTED.");
             return;
        }
        
        print!("Enter Password to Encrypt: ");
        io::stdout().flush().unwrap();
        let pass = rpassword::read_password().unwrap();
        
        print!("Confirm Password: ");
        io::stdout().flush().unwrap();
        let pass2 = rpassword::read_password().unwrap();
        
        if pass != pass2 {
            print_status("Passwords do not match!", true);
            return;
        }
        
        match vault::lock(&pass) {
            Ok(_) => print_status("VAULT SECURED. UNLOCKED DATA SHREDDED.", false),
            Err(e) => print_status(&format!("Error locking vault: {}", e), true),
        }
        
    } else {
        // Unlock Flow
        const MAX_ATTEMPTS: i32 = 3;
        let mut attempts = 0;
        
        loop {
            if attempts >= MAX_ATTEMPTS {
                print_status("SECURITY ALERT: MAX ATTEMPTS EXCEEDED.", true);
                println!("INITIATING EMERGENCY PROTOCOL...");
                let _ = vault::shred_vault();
                print_status("VAULT DESTROYED. DATA IS IRRECOVERABLE.", true);
                wait_for_key();
                break;
            }
            
            print!("ENTER PASSWORD TO UNLOCK: ");
            io::stdout().flush().unwrap();
            let pass = rpassword::read_password().unwrap();
            
            match vault::unlock(&pass) {
                Ok(true) => {
                    print_status("ACCESS GRANTED.", false);
                    println!("Files are available in 'UNLOCKED_CONTENTS'.");
                    println!("Run this program again to LOCK them when finished.");
                    wait_for_key();
                    break;
                },
                Ok(false) => {
                    attempts += 1;
                    print_status(&format!("ACCESS DENIED. Attempts remaining: {}", MAX_ATTEMPTS - attempts), true);
                },
                Err(e) => {
                     // Could be corrupt file or new vault creation error
                     // If error contains "Corrupted", behave like auth fail?
                     println!("System Error: {}", e);
                     break;
                }
            }
        }
    }
}

fn setup_terminal() {
    let _ = io::stdout().execute(SetTitle("HD-50 SECURE VAULT"));
    // Enable ANSI support on Windows? Crossterm handles it mostly.
}

fn print_banner() {
    let _ = io::stdout().execute(Clear(ClearType::All));
    let _ = io::stdout().execute(crossterm::cursor::MoveTo(0, 0));
    
    println!("==================================================");
    println!("          HD-50 PORTABLE SECURE VAULT             ");
    println!("==================================================");
    println!("");
}

fn print_status(msg: &str, is_error: bool) {
    if is_error {
        let _ = io::stdout().execute(SetForegroundColor(Color::Red));
        print!("[!] ");
    } else {
        let _ = io::stdout().execute(SetForegroundColor(Color::Green));
        print!("[+] ");
    }
    let _ = io::stdout().execute(ResetColor);
    println!("{}", msg);
}

fn wait_for_key() {
    println!("\nPress Enter to exit...");
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
}
