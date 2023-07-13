use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// read file tail, return Result<Vec<String>, String>
pub fn read_file_tail(
    file_path: impl AsRef<Path>,
    max_res_count: usize,
) -> Result<Vec<String>, String> {
    let file_lines_count = get_file_line_count(file_path.as_ref())?;
    if file_lines_count == 0 {
        return Err(format!("文件:{}无数据", file_path.as_ref().display()));
    }

    let file = File::open(file_path.as_ref()).map_err(|err| format!("文件不存在:{}", err))?;
    let lines = BufReader::new(file)
        .lines()
        .skip(file_lines_count.saturating_sub(max_res_count));
    let res_list = lines
        .map(|line| line.unwrap_or_else(|err| format!("读取文件出错:{}", err)))
        .collect::<Vec<String>>();
    Ok(res_list)
}

// get file total line count, return Result<usize, String>,
pub fn get_file_line_count(file_path: impl AsRef<Path>) -> Result<usize, String> {
    let file = File::open(file_path.as_ref()).map_err(|err| format!("文件不存在:{}", err))?;
    let reader = BufReader::new(file);
    let file_lines_count = reader.lines().count();
    Ok(file_lines_count)
}
