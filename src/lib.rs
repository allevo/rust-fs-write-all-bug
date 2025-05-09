use std::{io::{Seek, SeekFrom, Write}, os::unix::fs::FileExt, path::PathBuf};

pub fn run(file_path: PathBuf) {
    let print_file = |p: &'static str| {
        let buff = std::fs::read(&file_path).unwrap();
        println!("{p}: {buff:?}");
    };

    let mut page_file = std::fs::File::options()
        .create(true)
        .write(true)
        // .append(true)
        .read(true)
        .open(&file_path)
        .unwrap();

    let n: u32 = 10;
    let buf: [u8; 4] = n.to_be_bytes();
    page_file
        .write_all_at(&buf, 0).unwrap();
    print_file("after first write");

    page_file.flush().unwrap();
    page_file
        .sync_all().unwrap();

    let mut buf: [u8; 4] = [0; 4];
    page_file
        .read_exact_at(&mut buf, 0).unwrap();
    let n = u32::from_be_bytes(buf);
    print_file("after first read");

    page_file.seek(SeekFrom::Start(0)).unwrap();

    let buf: [u8; 4] = (n + 1).to_be_bytes();
    page_file
        .write_all_at(&buf, 0).unwrap();


    print_file("after second write");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::path::PathBuf;

    #[test]
    fn test_run() {
        let file_path = PathBuf::from("test_file");
        // Ignore the error if the file does not exist
        let _ = remove_file(&file_path);
        run(file_path.clone());

        let content = std::fs::read(&file_path)
            .unwrap();
        assert_eq!(content, vec![0, 0, 0, 11]);

        // Clean up the test file
        remove_file(file_path).unwrap();
    }
}
