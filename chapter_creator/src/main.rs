use std::fs;

fn main() -> std::io::Result<()> {
    for i in 4..22 {
        let res = format!("../Chapter-{}", i);
        let _ = fs::create_dir(res);
    }
    Ok(())
}

// https://doc.rust-lang.org/std/fs/fn.create_dir.html
