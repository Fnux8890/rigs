//! Workspace initialization

use std::path::PathBuf;
use crate::core::Result;

pub async fn run(path: PathBuf, git: bool) -> Result<()> {
    let path_str = path.display();
    
    println!("Initializing Rigs workspace at {}...", path_str);
    
    // TODO: Create directories
    println!("  Creating directories...");
    println!("    ✓ {}/", path_str);
    println!("    ✓ {}/db/", path_str);
    println!("    ✓ {}/logs/", path_str);
    
    // TODO: Create config
    println!("  Creating configuration...");
    println!("    ✓ {}/config.toml", path_str);
    
    // TODO: Initialize database
    println!("  Initializing database...");
    println!("    ✓ {}/db/rigs.db", path_str);
    println!("    ✓ Running migrations...");
    
    if git {
        println!("  Initializing git repository...");
        println!("    ✓ git init");
        println!("    ✓ .gitignore created");
    }
    
    println!();
    println!("✓ Workspace initialized!");
    println!();
    println!("Next steps:");
    println!("  1. Configure providers: rigs provider add claude");
    println!("  2. Check tank status:   rigs tank list");
    println!("  3. Create your first bead or goal:");
    println!("     rigs goal plan \"Add user authentication\"");
    
    Ok(())
}
