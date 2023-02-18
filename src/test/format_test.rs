use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

#[derive(Debug)]
struct Printtable(i32);

#[test]
fn format_test1() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{:#?}", Printtable(12))
}

#[test]
fn takke_no_test() {
    match File::open("E:\\97078\\Download\\debug.log") {
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            let res_file_path = "E:\\97078\\Download\\纠正信息.txt";
            File::create(res_file_path).expect("文件创建失败");
            let mut res_file = OpenOptions::new().append(true).open(res_file_path).unwrap();
            let mut is_match_line = 0;
            for line in lines {
                match line {
                    Ok(data) => {
                        if data.contains("纠正收件人地址信息/") {
                            is_match_line = 1;
                            let split_data: Vec<&str> = data.split("/").collect();
                            res_file
                                .write_all(format!("{};", split_data[1]).as_bytes())
                                .unwrap();
                            continue;
                        }
                        if is_match_line == 1 {
                            is_match_line = 2;
                            res_file.write_all(format!("{};", data).as_bytes()).unwrap();
                            continue;
                        }
                        if is_match_line == 2 {
                            is_match_line = 3;
                            res_file
                                .write_all(format!("{};\n", data).as_bytes())
                                .unwrap();
                            continue;
                        }
                    }
                    Err(err) => println!("{:#?}", err),
                }
            }
        }
        Err(err) => println!("{:#?}", err),
    }
}
