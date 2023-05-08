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

    let reader =
        BufReader::new(File::open(file_path).map_err(|err| format!("文件不存在:{}", err))?);
    let begin_read_index = if max_res_count > file_lines_count {
        0
    } else {
        file_lines_count - max_res_count
    };
    let mut res_list = Vec::with_capacity(max_res_count);
    let lines = reader.lines().skip(begin_read_index);
    for line in lines {
        match line {
            Ok(line_data) => {
                res_list.push(line_data);
            }
            Err(err) => {
                tracing::error!("读取文件出错:{}", err);
                return Err(format!("读取文件出错:{}", err));
            }
        }
    }
    //等待读取线程结束
    Ok(res_list)
}

// get file total line count, return Result<usize, String>,
pub fn get_file_line_count(file_path: impl AsRef<Path>) -> Result<usize, String> {
    // Open the file
    let file =
        File::open(file_path.as_ref()).map_err(|err| format!("File does not exist: {}", err))?;
    let reader = BufReader::new(file);

    // Read the number of lines in the file
    let file_lines_count = reader.lines().count();
    Ok(file_lines_count)
}
