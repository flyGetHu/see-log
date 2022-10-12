use crate::entity::ProjectModel;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load_log_file(project_model: ProjectModel) -> String {
    let (file_path, count) = match_model_info(project_model);
    match File::open(file_path) {
        Ok(file) => {
            return match read_file(file) {
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
                    res_str
                }
                Err(err) => err,
            }
        }
        Err(err) => {
            tracing::error!("打开文件失败:{}", err);
            format!("打开文件失败:{}", err)
        }
    }
}

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

///匹配项目模块信息
fn match_model_info(project_model: ProjectModel) -> (&'static str, usize) {
    let mut file_path = "";
    let mode_name = project_model.mode_name;
    let count = project_model.count;
    if mode_name == "express" {
        file_path = "/home/work/express-app/express-app.log"
        // file_path = "C:\\Users\\97078\\Desktop\\fsdownload\\error.log"
    } else if mode_name == "admin-oa" {
        file_path = "/home/work/admin-oa/admin-oa.log"
    }
    (file_path, count)
}
