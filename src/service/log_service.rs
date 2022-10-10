use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load_log_file(project_model: crate::entity::ProjectModel) -> String {
    let mut file_path = "";
    let mode_name = project_model.mode_name;
    let count = project_model.count;
    if mode_name == "express" {
        file_path = "/home/work/express-app/express-app.log"
        // file_path = "C:\\Users\\97078\\Desktop\\fsdownload\\error.log"
    } else if mode_name == "admin-oa" {
        file_path = "/home/work/admin-oa/admin-oa.log"
    }
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut res_list = Vec::with_capacity(1024);
    for line in lines {
        let line_res = line.unwrap();
        res_list.push(line_res)
    }
    let res_len = res_list.len();
    let mut start: usize = 0;
    if res_len > count {
        start = res_len - count;
    }
    let res_list_final = &res_list[start..];
    let mut res_str = String::new();
    for line in res_list_final {
        res_str += &format!("{}\n", line);
    }
    res_str
}
