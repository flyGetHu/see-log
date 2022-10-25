use crate::entity::LogParam;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

///读取日志文件并返回指定字符串
pub fn load_log_file(project_model: LogParam) -> Result<String, String> {
    let count = project_model.count;
    let file_path = project_model.file_path;
    if !file_path.ends_with(".log") {
        return Err(format!("当前只允许查看日志类型文件:{}", file_path));
    }
    match File::open(&file_path) {
        Ok(file) => match read_file(file) {
            Ok(data) => {
                let res_len = data.len();
                let mut start: usize = 0;
                if res_len > count {
                    start = res_len - count;
                }
                let res_list_final = &data[start..];
                let mut res_str = String::new();
                for line in res_list_final {
                    res_str += &format!("{}\n", line);
                }
                Ok(res_str)
            }
            Err(err) => Err(err),
        },
        Err(err) => {
            let msg = format!("{}:{}", err, &file_path);
            tracing::error!("打开文件失败:{}", msg);
            Err(format!("打开文件失败:{}", msg))
        }
    }
}

//读取指定文件,按行存入vec
fn read_file(file: File) -> Result<Vec<String>, String> {
    let mut reader = BufReader::new(file);
    let mut line_data: String;
    let mut res_list = Vec::with_capacity(1024);
    loop {
        line_data = String::from("");
        match reader.read_line(&mut line_data) {
            Ok(data_size) => {
                //代表已完成读取
                if data_size == 0 {
                    break;
                }
                res_list.push(line_data)
            }
            Err(err) => {
                tracing::error!("读取文件行数出错:{}", err);
                return Err(format!("读取文件行数出错:{}", err));
            }
        }
    }
    Ok(res_list)
}
