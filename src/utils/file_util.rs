use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

//定义消息类型
enum Message {
    Line(String),
    End,
}

// read file tail, return Result<Vec<String>, String>
pub fn read_file_tail(file_path: impl AsRef<Path>, max_res_count: usize) -> Result<Vec<String>, String> {
    let file_lines_count = get_file_line_count(file_path.as_ref())?;
    if file_lines_count == 0 {
        return Err(format!("文件:{}无数据", file_path.as_ref().display()));
    }

    let (tx, rx): (Sender<Message>, Receiver<Message>) = channel(); //定义通道

    let reader = BufReader::new(File::open(file_path).map_err(|err| format!("文件不存在:{}", err))?);
    let begin_read_index = if max_res_count > file_lines_count {
        0
    } else {
        file_lines_count - max_res_count
    };

    //启动一个线程逐行读取文件内容
    let read_handle = thread::spawn(move || {
        let mut lines = reader.lines();
        for (index, line) in lines.by_ref().enumerate() {
            if index >= begin_read_index {
                match line {
                    Ok(line_data) => {
                        if tx.send(Message::Line(line_data)).is_err() {
                            tracing::error!("发送消息出错");
                            break;
                        }
                    }
                    Err(err) => {
                        tracing::error!("读取文件出错:{}", err);
                    }
                }
            }
        }
        let _ = tx.send(Message::End); //发送结束消息
    });

    let mut res_list = Vec::with_capacity(max_res_count);
    //处理消息队列中的数据
    for msg in rx {
        match msg {
            Message::Line(line_data) => {
                res_list.push(line_data);
            }
            Message::End => {
                break;
            }
        }
    }
    //等待读取线程结束
    read_handle.join().unwrap();
    Ok(res_list)
}

// get file total line count, return Result<usize, String>,
pub fn get_file_line_count(file_path: impl AsRef<Path>) -> Result<usize, String> {
    let file_content = std::fs::read_to_string(file_path.as_ref()).map_err(|err| format!("文件不存在:{}", err))?;
    let file_lines_count = file_content.lines().count();
    Ok(file_lines_count)
}
