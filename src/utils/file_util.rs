use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek};
use std::path::Path;

/// Reads the last `max_res_count` lines of a file at `file_path`.
/// If the file size is less than or equal to 10MB, it reads the entire file.
/// Returns a `VecDeque` containing the lines read, with a maximum capacity of `max_res_count`.
pub fn read_file_tail(
    file_path: impl AsRef<Path>,
    max_res_count: usize,
) -> io::Result<VecDeque<String>> {
    let max_size = 1024;
    let metadata = match std::fs::metadata(&file_path) {
        Ok(metadata) => metadata,
        Err(err) => return Err(err),
    };
    let file_size = metadata.len();
    if file_size <= max_size {
        match read_file_tail_all(file_path, max_res_count) {
            Ok(data) => return Ok(data),
            Err(err) => return Err(err),
        }
    }
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut reader = BufReader::new(file);
    let mut res_vec = VecDeque::with_capacity(max_res_count);
    let mut pos = file_size - max_size;
    let mut buf = String::new();
    loop {
        match reader.seek(std::io::SeekFrom::Start(pos)) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                if res_vec.len() >= max_res_count {
                    res_vec.pop_front();
                }
                res_vec.push_back(buf.clone());
                buf.clear();
            }
            Err(err) => return Err(err),
        }
        pos += buf.len() as u64;
    }
    Ok(res_vec)
}

/// Reads all lines of a file at `file_path`.
/// Returns a `VecDeque` containing the lines read, with a maximum capacity of `max_res_count`.
pub fn read_file_tail_all(
    file_path: impl AsRef<Path>,
    max_res_count: usize,
) -> io::Result<VecDeque<String>> {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut res_deque: VecDeque<String> = VecDeque::with_capacity(max_res_count);
    for line_res in lines {
        match line_res {
            Ok(line) => {
                if res_deque.len() >= max_res_count {
                    res_deque.pop_front();
                }
                res_deque.push_back(line);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    Ok(res_deque)
}
