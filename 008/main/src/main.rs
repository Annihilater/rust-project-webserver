use std::{fs, io, path::Path};

fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name();

            if path.is_dir() {
                let result = visit_dirs(&path);
                println!("dir: result = {:?}", result);
            } else {
                let c = fs::read_to_string(path).unwrap();
                println!("file {} = {}", file_name.to_string_lossy(), c);
            }
        }
    }

    Ok(())
}

fn main() {
    let context = fs::read("./test/tt1").unwrap();
    println!("context = {:#?}", context);

    let context = fs::read_to_string("./test/tt1").unwrap();
    println!("context = {:#?}", context);

    visit_dirs(Path::new("./test")).unwrap();
}
