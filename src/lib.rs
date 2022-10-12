pub mod entity;
pub mod route;
pub mod service;
pub mod handle;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn it_works() {
        let path = "C:\\Users\\97078\\Desktop\\fsdownload\\access.log";
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();
        let mut res = Vec::new();
        for line in lines {
            res.push(line.unwrap())
        }
        let count = 1024;
        let mut start = 0;
        let length = res.len();
        if length > count {
            start = length - count;
        }
        let res_final = &res[start..length];
        for line in res_final {
            println!("{}", line)
        }
    }

    #[test]
    fn enum_test() {
        #[derive(Debug)]
        enum ProjectModel {
            Express,
        }
        println!("{:?}", ProjectModel::Express)
    }

    #[test]
    fn file_read_test() {
        let path = "C:\\Users\\97078\\Desktop\\fsdownload\\access.log";
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut buf: String = String::from("");
        loop {
            buf.clear();
            let size = reader.read_line(&mut buf).expect("TODO: panic message");
            if buf.len() != 0 {
                println!("{}", buf)
            } else {
                println!("{size}");
                break;
            }
        }
    }
}
