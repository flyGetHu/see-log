use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn load_log_file(file_path: &str, count: usize) -> String {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut res_list = Vec::with_capacity(1024);
    for line in lines {
        let line_res = line.unwrap();
        res_list.push(line_res)
    }
    let start = res_list.len() - count;
    let res_list_final = &res_list[start..];
    let mut res_str = String::new();
    for line in res_list_final {
        res_str += &format!("{}\n", line);
    }
    res_str
}
