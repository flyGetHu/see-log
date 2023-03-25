use crate::utils::file_util;

#[derive(Debug)]
pub struct LogFile {
    //读取日志文件目录
    pub file_path: String,
    //读取日志文件最后多少行
    pub count: usize,
}

impl LogFile {
    ///读取日志文件并返回指定字符串
    pub fn load_log_file(self) -> Result<String, String> {
        let count = self.count;
        let file_path = self.file_path;
        if !file_path.ends_with(".log") {
            return Err(format!("当前只允许查看日志类型文件:{}", file_path));
        }
        match file_util::read_file_tail(file_path, count) {
            Ok(data) => {
                let res_str = data.join("\n");
                Ok(res_str)
            }
            Err(err) => Err(err),
        }
    }
}
