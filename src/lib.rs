pub mod route;
pub mod service;


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
        let start = res.len() - 1024;
        let res_final = &res[start..res.len()];
        for line in res_final {
            println!("{}", line)
        }
    }
}
