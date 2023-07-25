use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Read the last few lines of a file
///
/// # Arguments
///
/// * `file_path` - File path
/// * `max_res_count` - Maximum number of lines to read
///
/// # Returns
///
/// A `Result` containing a `Vec` of `String`s representing the last few lines of the file.
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be opened or read.
pub fn read_file_tail(
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
