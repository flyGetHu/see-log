use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

#[test]
fn test_read_file_tail() {
    let file_path = "D:\\workspace\\golang\\anjun-desktop-plug\\logs\\errors.log";
    let file = File::open(file_path).unwrap();
    let file_len = file.metadata().unwrap().len() as usize;
    let mut reader = BufReader::new(file);
    let max_res_count = 10;
    let mut res_list = Vec::with_capacity(max_res_count);
    //读取最后几行
    let mut pos = (file_len - max_res_count) as usize;
    while pos > 0 && res_list.len() < max_res_count {
        let mut line = String::new();

        // Seek to the start of the previous line
        reader.seek(SeekFrom::Start(pos as u64)).unwrap();
        reader.read_line(&mut line).unwrap();
        pos -= line.len();
        if pos == file_len {
            break;
        }
        res_list.push(line.trim_end().to_string());
    }
    res_list.reverse(); // The lines are in reverse order, so we reverse them
    println!("{:?}", res_list);
}
