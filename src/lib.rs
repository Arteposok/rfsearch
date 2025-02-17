use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;

pub fn parse_files(path: String, search_for: String, file_ext: String) {
    let threads: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(Vec::new()));
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_dir() {
                let sf = search_for.clone();
                let ext = file_ext.clone();
                let created = thread::spawn(move || {
                    parse_files(String::from(file_path.to_str().unwrap()), sf, ext);
                });
                threads.lock().unwrap().push(created);
            } else if let Ok(lines) = fs::read_to_string(file_path.clone()) {
                if !is_text(file_path.to_str().unwrap()).unwrap() {
                    return;
                }
                if String::from(file_path.file_name().unwrap().to_str().unwrap())
                    .ends_with(&file_ext)
                    || file_ext == "_"
                {
                    for (num, line) in lines.lines().enumerate() {
                        if line.contains(search_for.as_str()) {
                            println!(
                                "====> peeking inside the file {}",
                                file_path.to_str().unwrap()
                            );
                            println!();
                            println!("  |{} | {}", num + 1, line);
                            println!();
                            println!("________________________________________________________________________________________");
                            println!();
                        }
                    }
                }
            }
        }
    }

    if let Ok(mut handles) = Arc::clone(&threads).lock() {
        while let Some(handle) = handles.pop() {
            handle.join().unwrap();
        }
    }
}

fn is_text(file_path: &str) -> io::Result<bool> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0; 1024];
    let bytes_read = file.read(&mut buffer)?;
    let non_printables = buffer[..bytes_read]
        .iter()
        .filter(|&&b| !b.is_ascii() && b < 0x20 && b != b'\n' && b != b'\r' && b != b'\t')
        .count();
    Ok(non_printables < (bytes_read / 20))
}
