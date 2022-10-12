use crate::entity::ProjectModel;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

///读取日志文件并返回指定字符串
pub fn load_log_file(project_model: ProjectModel) -> Result<String, String> {
    match match_model_info(project_model) {
        Ok((file_path, count)) => match File::open(&file_path) {
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
                Err(err) => err,
            },
            Err(err) => {
                let msg = format!("{}:{}", err, &file_path);
                tracing::error!("打开文件失败:{}", msg);
                format!("打开文件失败:{}", msg)
            }
        },
        Err(err) => err,
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

///匹配项目模块信息
fn match_model_info(project_model: ProjectModel) -> Result<(String, usize), String> {
    let mut file_path = String::from("");
    let mode_name = project_model.mode_name;
    let count = project_model.count;
    let log_level = project_model.log_level;
    if mode_name == "local" {
        //本地测试
        file_path = String::from("C:\\Users\\97078\\Desktop\\fsdownload\\error.log")
    } else if mode_name == "express" {
        //海外项目
        file_path = String::from("/home/work/express-app/express-app.log")
    } else if mode_name == "z-manage" {
        //国内管理端
        file_path = String::from("/home/work/admin-oa/admin-oa.log")
    } else if mode_name == "z-warehouse" {
        //国内仓储
        file_path = format!("/home/work/anjun-warehouse-server/logs/{}.log", log_level);
    } else {
        return Err(format!(
            "请检查参数是否异常:{},{},{}",
            mode_name, count, log_level
        ));
    }
    Ok((file_path, count))
}
