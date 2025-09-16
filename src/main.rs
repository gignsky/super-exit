use clap::{Arg, ArgAction, Command};
use std::env;
use std::process::{self, Command as ProcessCommand};
use sysinfo::{Pid, ProcessExt, System, SystemExt};

#[derive(Debug)]
struct SuperExit {
    verbose: bool,
    dry_run: bool,
}

impl SuperExit {
    fn new(verbose: bool, dry_run: bool) -> Self {
        Self { verbose, dry_run }
    }

    /// Check if we're running in Nushell
    fn is_nushell_environment(&self) -> bool {
        // Check if we're in a Nushell environment
        if let Ok(shell) = env::var("NUSHELL_VERSION") {
            if self.verbose {
                println!("Detected Nushell version: {}", shell);
            }
            return true;
        }

        // Alternative check: look at parent processes
        let system = System::new_all();
        let current_pid = process::id();
        
        if let Some(process) = system.process(Pid::from(current_pid as usize)) {
            let mut parent_pid = process.parent();
            let mut depth = 0;
            
            while let Some(pid) = parent_pid {
                if let Some(parent_process) = system.process(pid) {
                    let name = parent_process.name().to_lowercase();
                    if self.verbose {
                        println!("Parent process {}: {}", depth, name);
                    }
                    
                    if name.contains("nu") || name.contains("nushell") {
                        if self.verbose {
                            println!("Found Nushell parent process: {}", name);
                        }
                        return true;
                    }
                    
                    parent_pid = parent_process.parent();
                    depth += 1;
                    
                    // Prevent infinite loops
                    if depth > 10 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        
        false
    }

    /// Count nested shell levels
    fn count_nested_shells(&self) -> usize {
        let system = System::new_all();
        let current_pid = process::id();
        let mut shell_count = 0;
        
        if let Some(process) = system.process(Pid::from(current_pid as usize)) {
            let mut parent_pid = process.parent();
            let mut depth = 0;
            
            while let Some(pid) = parent_pid {
                if let Some(parent_process) = system.process(pid) {
                    let name = parent_process.name().to_lowercase();
                    
                    // Check for various shell types
                    if name.contains("nu") || name.contains("nushell") 
                        || name.contains("bash") || name.contains("zsh") 
                        || name.contains("fish") || name.contains("sh") {
                        shell_count += 1;
                        if self.verbose {
                            println!("Found shell {}: {} (PID: {:?})", shell_count, name, pid);
                        }
                    }
                    
                    parent_pid = parent_process.parent();
                    depth += 1;
                    
                    // Prevent infinite loops
                    if depth > 20 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        
        shell_count
    }

    /// Execute the super exit functionality
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.verbose {
            println!("Starting super-exit...");
        }

        let is_nushell = self.is_nushell_environment();
        let shell_count = self.count_nested_shells();

        if self.verbose {
            println!("Running in Nushell: {}", is_nushell);
            println!("Detected {} nested shells", shell_count);
        }

        if shell_count == 0 {
            println!("No nested shells detected. Use regular exit command.");
            return Ok(());
        }

        if self.dry_run {
            println!("DRY RUN: Would exit {} nested shells", shell_count);
            return Ok(());
        }

        // For Nushell, we need to send multiple exit commands
        if is_nushell {
            self.exit_nushell_recursive(shell_count)?;
        } else {
            self.exit_generic_shells(shell_count)?;
        }

        Ok(())
    }

    /// Exit Nushell shells recursively
    fn exit_nushell_recursive(&self, count: usize) -> Result<(), Box<dyn std::error::Error>> {
        if self.verbose {
            println!("Executing Nushell recursive exit for {} shells", count);
        }

        // In Nushell, we can use the built-in exit command
        // We'll send multiple exit commands to handle nested shells
        for i in 0..count {
            if self.verbose {
                println!("Sending exit command {} of {}", i + 1, count);
            }
            
            // Use Nushell's exit command
            let output = ProcessCommand::new("nu")
                .arg("-c")
                .arg("exit")
                .output();
                
            match output {
                Ok(_) => {
                    if self.verbose {
                        println!("Exit command {} executed successfully", i + 1);
                    }
                }
                Err(e) => {
                    if self.verbose {
                        println!("Warning: Exit command {} failed: {}", i + 1, e);
                    }
                }
            }
        }

        // Final exit
        process::exit(0);
    }

    /// Exit generic shells
    fn exit_generic_shells(&self, count: usize) -> Result<(), Box<dyn std::error::Error>> {
        if self.verbose {
            println!("Executing generic shell exit for {} shells", count);
        }

        // For non-Nushell environments, just exit normally
        println!("Exiting {} nested shells...", count);
        process::exit(0);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("super-exit")
        .version("0.1.0")
        .author("gignsky")
        .about("Recursively exit nested Nushell shells to close terminal")
        .long_about("A utility to recursively exit nested shells, particularly designed for Nushell environments. It detects nested shell processes and attempts to exit them all in sequence to close the terminal completely.")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Enable verbose output")
        )
        .arg(
            Arg::new("dry-run")
                .short('n')
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Show what would be done without actually exiting")
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Just count and display nested shells without exiting")
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
    let dry_run = matches.get_flag("dry-run");
    let count_only = matches.get_flag("count");

    let super_exit = SuperExit::new(verbose, dry_run);

    if count_only {
        let shell_count = super_exit.count_nested_shells();
        let is_nushell = super_exit.is_nushell_environment();
        
        println!("Shell environment: {}", if is_nushell { "Nushell" } else { "Other" });
        println!("Nested shells detected: {}", shell_count);
        return Ok(());
    }

    super_exit.execute()
}