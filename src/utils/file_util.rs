use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// read file tail, return Result<Vec<String>, String>
pub fn read_file_tail(file_path: impl AsRef<Path>, max_res_count: usize) -> Result<Vec<String>, String> {
    let file = File::open(file_path.as_ref()).map_err(|err| format!("文件不存在: {}", err))?;
    let reader = BufReader::new(file);

    let mut res_list = Vec::with_capacity(max_res_count);
    let mut lines = reader.lines();

    // Skip lines until we reach the desired starting position
    let mut skipped_lines = 0;
    while let Some(result) = lines.next() {
        match result {
            Ok(line_data) => {
                if skipped_lines >= max_res_count {
                    res_list.remove(0); // Remove the oldest line if we have reached the maximum count
                } else {
                    skipped_lines += 1;
                }
                res_list.push(line_data);
            }
            Err(err) => {
                return Err(format!("读取文件出错: {}", err));
            }
        }
    }
    Ok(res_list)
}
