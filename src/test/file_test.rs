use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use salvo::http::uri::Port;

#[test]
fn read_file_test() {
    let file_path = String::from("D:\\home\\work\\admin-app\\logs\\admin-app_2022-11-01.log");
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        println!("行数:{}|{}", index, line.unwrap())
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[test]
fn test4() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // 以 Debug 方式打印结构体
    println!("{:?}", peter);

    // 实例化结构体 `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 point，
    // 这样可以用到之前的 point 的字段
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用 `let` 绑定来解构 point
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

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
