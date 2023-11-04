// [read_dir in std::fs](https://doc.rust-lang.org/std/fs/fn.read_dir.html) に２つあるサンプルコードの下の方をベースにしました。

use std::{fs, io};

fn main() -> io::Result<()> {
    let entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in &entries {
        let entry = entry.as_path();
        let file_name = entry.file_name().unwrap().to_str().unwrap();
        if !file_name.starts_with(".") {
            print!("{}     ", file_name);
        }
    }
    println!("");

    Ok(())
}
