use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek};
use std::path::Path;

// 读取文件超过指定大小时，使用seek方式读取
const MAX_LINE_LENGTH: u64 = 1024 * 1024 * 10;

/// Reads the last `max_res_count` lines of the file.
/// If the file size is less than or equal to max_size, it reads the entire file.
pub fn read_file_tail(
    file_path: impl AsRef<Path>,
    max_res_count: usize,
) -> io::Result<VecDeque<String>> {
    let metadata = match std::fs::metadata(&file_path) {
        Ok(metadata) => metadata,
        Err(err) => return Err(err),
    };
    let file_size = metadata.len();
    // If the file size is less than or equal to max_size, it reads the entire file.
    if file_size <= MAX_LINE_LENGTH {
        return match read_file_tail_all(file_path, max_res_count) {
            Ok(data) => Ok(data),
            Err(err) => Err(err),
        };
    }
    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut reader = BufReader::new(file);
    let mut res_vec = VecDeque::with_capacity(max_res_count);
    // Start reading from the end of the file. 只读取最后10MB的内容
    let mut pos = file_size - MAX_LINE_LENGTH;
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
                // `buf` is now empty, so we can reuse it.
                pos += buf.len() as u64;
                buf.clear();
            }
            Err(err) => return Err(err),
        }
    }
    Ok(res_vec)
}

/// Reads the entire file.
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
