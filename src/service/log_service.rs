use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load_log_file(project_model: crate::entity::ProjectModel) -> String {
    let mut file_path = "";
    let mode_name = project_model.mode_name;
    let count = project_model.count;
    if mode_name == "express" {
        // file_path = "/home/work/express-app/express-app.log"
        file_path = "C:\\Users\\97078\\Desktop\\fsdownload\\error.log"
    } else if mode_name == "admin-oa" {
        file_path = "/home/work/admin-oa/admin-oa.log"
    }
    let mut res_str = String::new();
    match File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let lines = reader.lines();
            let mut res_list = Vec::with_capacity(1024);
            for line in lines {
                match line {
                    Ok(line_data) => {
                        res_list.push(line_data)
                    }
                    Err(err) => {
                        println!("读取文件行数出错:{}", err);
                        return format!("读取文件行数出错:{}", err);
                    }
                }
            }
            let res_len = res_list.len();
            let mut start: usize = 0;
            if res_len > count {
                start = res_len - count;
            }
            let res_list_final = &res_list[start..];
            for line in res_list_final {
                res_str += &format!("{}\n", line);
            }
        }
        Err(err) => {
            println!("打开文件失败:{}", err);
            return format!("打开文件失败:{}", err);
        }
    }
    res_str
}
