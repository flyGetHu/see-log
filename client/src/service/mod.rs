use std::error::Error;
use std::fs::{metadata, read_dir};

// 读取所有日志文件
fn load_log_files(root_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut path_list = vec![String::from(root_path)];
    let mut start_index = 0;
    loop {
        let list_len = path_list.len();
        for index in start_index..path_list.len() {
            let path = &path_list[index];
            match metadata(path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        for child_dir_res in read_dir(&path)? {
                            match child_dir_res {
                                Ok(child_dir) => {
                                    let file_path = String::from(
                                        child_dir
                                            .path()
                                            .as_os_str()
                                            .to_str()
                                            .expect("无效的文件路径"),
                                    );
                                    path_list.push(file_path);
                                }
                                Err(err) => {
                                    println!("无效的目录文件:{}", err)
                                }
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
        if list_len == start_index {
            break;
        }
        start_index = list_len;
    }
    return Ok(path_list);
}

#[test]
fn load_log_files_test() {
    let files = load_log_files(crate::common::PROJECT_LOG_ROOT_PATH).expect("错误");
    for file in files {
        println!("{:?}", file)
    }
}
