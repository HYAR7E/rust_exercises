use std::{
    fs::File,
    io::{Read, Write},
    ops::Drop,
    path::PathBuf,
};
use tempdir::TempDir;

pub struct DurableFile {
    file: File,
    need_sync: bool,
}

fn create_temp_dir() -> TempDir {
    TempDir::new("rust_tmp").unwrap()
}

impl DurableFile {
    pub fn new(filename: &str) -> Self {
        let file = if filename.contains('/') {
            let filepath = PathBuf::from(filename);
            File::open(filepath).unwrap()
        } else {
            let temp_dir = create_temp_dir();
            let filepath = temp_dir.path().join(filename);
            File::create(filepath.clone()).unwrap()
        };
        DurableFile {
            file,
            need_sync: false,
        }
    }

    pub fn read(&mut self) -> String {
        let mut content = String::new();
        self.file.read_to_string(&mut content).unwrap();
        content.trim().to_string()
    }

    pub fn close(mut self) {
        if self.need_sync {
            self.flush();
        };
    }
}

impl From<File> for DurableFile {
    fn from(file: File) -> Self {
        DurableFile {
            file,
            need_sync: false,
        }
    }
}

impl Write for DurableFile {
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.need_sync = true;
        self.file.write_all(buf)
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.need_sync = true;
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let result = self.file.flush();
        match result {
            Ok(_) => {
                self.need_sync = false;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl Drop for DurableFile {
    fn drop(&mut self) {
        if self.need_sync {
            panic!("File not flushed!");
        }
        drop(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn creates_file_if_not_exists() {
        DurableFile::new("data.txt");
    }

    #[test]
    fn opens_file_if_receives_path() {
        let mut file = DurableFile::new(
            "/Users/agustinf/Documents/repositories/learning/rust/durable_file/data.txt",
        );
        assert_eq!(file.read(), String::from("Hello World!"));
    }

    #[test]
    #[should_panic(expected = "File not flushed!")]
    fn panics_if_not_flushed() {
        let mut durable = DurableFile::new("data.txt");
        durable.write_all("Hello World!".as_bytes()).unwrap();
    }

    #[test]
    fn closes_correctly() {
        let durable = DurableFile::new("data.txt");
        durable.close();
    }

    #[test]
    #[should_panic(expected = "No such file")]
    fn close_panics_when_file_is_deleted() {
        let filepath =
            "/Users/agustinf/Documents/repositories/learning/rust/durable_file/deleted.txt";
        let mut durable = DurableFile::new(filepath);
        fs::remove_file(filepath).unwrap();
        durable.write_all("Hello World!".as_bytes()).unwrap();
        durable.close();
    }

    #[test]
    fn happy_path() {
        let mut durable = DurableFile::new("data.txt");
        durable.write_all("Hello World!".as_bytes()).unwrap();
        durable.close();
    }
}
