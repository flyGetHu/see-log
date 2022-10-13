#[derive(Debug)]
pub struct LogParam {
    //读取日志文件目录
    pub file_path: String,
    //读取日志文件最后多少行
    pub count: usize,
}
