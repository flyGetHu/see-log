use std::error::Error;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};

// 读取所有日志文件 只获取第二层文件的日志文件
fn load_log_files(
    root_path: &str,
    path_list: &mut Vec<String>,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file_path_list: Vec<String> = vec![];
    let dir_list = read_dir(root_path)?;
    for dir in dir_list {
        let entity = dir?;
        if entity.metadata()?.is_dir() {
            let path_buf = entity.path();
            match path_buf.as_os_str().to_str() {
                None => {}
                Some(dir_path) => file_path_list.push(String::from(dir_path)),
            }
        }
    }
    for file_path in file_path_list {
        let read_dir = read_dir(&file_path)?;
        for entry_result in read_dir {
            let entry = entry_result?;
            if entry.metadata()?.is_file() {
                match entry.path().to_str() {
                    None => {}
                    Some(path_str) => {
                        if path_str.ends_with(".log") {
                            path_list.push(path_str.to_string())
                        }
                    }
                }
            }
        }
    }
    return Ok(path_list.to_owned());
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
                println!("读取文件行数出错:{}", err);
                return Err(format!("读取文件行数出错:{}", err));
            }
        }
    }
    Ok(res_list)
}

#[test]
fn load_log_files_test() {
    let mut path_list: Vec<String> = vec![];
    match load_log_files(crate::common::PROJECT_LOG_ROOT_PATH, &mut path_list) {
        Ok(files) => {
            for file_path in files {
                println!("{:?}", file_path);
                match File::open(file_path) {
                    Ok(file) => {
                        let file_data = read_file(file);
                        println!("{:?}", file_data)
                    }
                    Err(_) => todo!(),
                }
            }
        }
        Err(_) => {}
    };
}
