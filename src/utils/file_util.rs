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
) -> io::Result<Vec<String>> {
    let file = File::open(file_path.as_ref())?;
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut res_vec: Vec<String> = Vec::with_capacity(max_res_count);
    let mut end_line = String::new();
    for line_res in lines {
        match line_res {
            Ok(line) => {
                if res_vec.len() >= max_res_count {
                    end_line = res_vec[0].clone();
                    res_vec.remove(0);
                }
                res_vec.push(line);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    if res_vec.len() < max_res_count {
        res_vec.insert(0, end_line);
    }
    Ok(res_vec)
}
