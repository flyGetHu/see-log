use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
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
  let mut file = File::open(file_path.as_ref())?;
  let file_len = file.metadata()?.len();
  // Seek to the end of the file
  file.seek(SeekFrom::End(0))?;

  let mut reader = BufReader::new(file);
  let mut res_list = Vec::with_capacity(max_res_count);
  let mut pos = file_len as usize;

  // Read lines from the end of the file
  while pos > 0 && res_list.len() < max_res_count {
    let mut line = String::new();
    match reader.read_line(&mut line) {
      Ok(_) => {
        res_list.push(line.clone());
      }
      Err(err) => {
        return Err(err);
      }
    }
    pos -= line.len();
    // Seek to the start of the previous line
    reader.seek(SeekFrom::Start(pos as u64))?;
  }
  res_list.reverse(); // The lines are in reverse order, so we reverse them
  Ok(res_list)
}
