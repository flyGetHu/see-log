use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::entity::LogParam;

///读取日志文件并返回指定字符串
pub fn load_log_file(project_model: LogParam) -> Result<String, String> {
    let count = project_model.count;
    let file_path = project_model.file_path;
    if !file_path.ends_with(".log") {
        return Err(format!("当前只允许查看日志类型文件:{}", file_path));
    }
    return match read_file(file_path, count) {
        Ok(data) => {
            let mut res_str = String::new();
            for line in data {
                res_str += &format!("{}\n", line);
            }
            Ok(res_str)
        }
        Err(err) => Err(err),
    };
}

//读取指定文件,按行存入vec
fn read_file(file_path: String, max_res_count: usize) -> Result<Vec<String>, String> {
    let file_lines_count = match File::open(&file_path) {
        Ok(file) => BufReader::new(file).lines().count(),
        Err(err) => {
            tracing::error!("读取文件出错:{}", err);
            0
        }
    };
    if file_lines_count == 0 {
        return Err(format!("文件:{}不存在", file_path));
    }
    let reader = BufReader::new(File::open(&file_path).unwrap());
    //文件总行数-最大可查看行数 获取绝对值 读取文件时从此处开始存入数据
    let difference: isize = (file_lines_count - max_res_count) as isize;
    let begin_read_index = num::abs(difference);
    let mut res_list = Vec::with_capacity(max_res_count);
    for (index, line) in reader.lines().enumerate() {
        if index < begin_read_index as usize {
            continue;
        }
        match line {
            Ok(line_data) => {
                res_list.push(line_data);
            }
            Err(err) => {
                tracing::error!("读取文件行数出错:{}", err);
                return Err(format!("读取文件行数出错:{}", err));
            }
        }
    }
    Ok(res_list)
}
