use std::fs::File;
use std::io::{BufRead, BufReader};

//读取指定文件,按行存入vec 读取文件最后多少行
pub fn read_file_tail(file_path: String, max_res_count: usize) -> Result<Vec<String>, String> {
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
    //从文件开始行读取,其他行跳过
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