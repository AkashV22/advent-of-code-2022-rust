use std::{
    fs::File,
    io,
    io::{BufRead, BufReader, Lines},
    path::PathBuf,
};

pub(crate) trait InputLoader {
    fn load_input_to_lines(self) -> io::Result<Lines<BufReader<File>>>;
}

impl InputLoader for &str {
    fn load_input_to_lines(self) -> io::Result<Lines<BufReader<File>>> {
        let input_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input", self].iter().collect();

        let file: File = File::open(input_path)?;
        let buf: BufReader<File> = BufReader::new(file);

        Ok(buf.lines())
    }
}
