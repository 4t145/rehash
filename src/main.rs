use std::io::Write;
fn main() {
    let mut open_opt = std::fs::OpenOptions::new();
    open_opt.write(true).append(true);
    for file_name in std::env::args_os() {
        if let Ok(mut file) = open_opt.open(file_name.clone()) {
            let mut random_append = rand::random::<[u8;31]>();
            random_append[0] = b'\n';
            random_append[1] = 255;
            match file.write(&random_append) {
                Ok(_) => println!("Ok on {}", file_name.to_str().unwrap_or_default()),
                Err(_) => println!("Fail on {}", file_name.to_str().unwrap_or_default()),
            }
        }
    }
}
