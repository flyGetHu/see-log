use std::error::Error;
use std::fs::read_dir;

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

#[test]
fn load_log_files_test() {
    let mut path_list: Vec<String> = vec![];
    match load_log_files(crate::common::PROJECT_LOG_ROOT_PATH, &mut path_list) {
        Ok(files) => {
            for file in files {
                println!("{:?}", file)
            }
        }
        Err(_) => {}
    };
}
