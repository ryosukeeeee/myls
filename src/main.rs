// メソッドチェーン & ?演算子による早期リターンをふんだんに使った例

use std::{fs, io};

fn main() -> io::Result<()> {
    fs::read_dir(".")?
        .map(|res| {
            res.map(|entry| {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();

                if !file_name.starts_with(".") {
                    print!("{}     ", file_name);
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    println!("");

    Ok(())
}
