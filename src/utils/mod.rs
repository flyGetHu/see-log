pub mod file_util;

#[cfg(test)]
mod tests {
    use crate::utils::file_util;

    #[test]
    fn test_read_file_tail() {
        let result = file_util::read_file_tail("D:\\home\\work\\anjun-takeno-server\\logs\\info.log", 100);
        match result {
            Ok(data) => {
                for line in data {
                    println!("{}", line);
                }
            }
            Err(_) => {}
        }
    }
}