use std::rc::Rc;
use std::thread;

#[test]
fn test1() {
    let rc_example = "RcExample".to_string();
    {
        println!("rc_example is created: {}", rc_example);
        let rc_a = Rc::new(rc_example);
        println!("references: {}", Rc::strong_count(&rc_a));
        {
            println!("rc_a is cloned to rc_b");

            let rc_b = Rc::clone(&rc_a);

            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // 如果两者内部的值相等的话，则两个 `Rc` 相等。
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // 我们可以直接使用值的方法
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }
}

#[test]
fn test2() {
    let count = 10;
    let mut children = vec![];

    for i in 0..count {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }
    for child in children {
        child.join().unwrap();
    }
}

#[test]
fn test3() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
    let mut chunked_data = vec![];
    for data in data.split_whitespace() {
        chunked_data.push(data.trim());
    }
    let step = 2;
    let lines = chunked_data.len();
    let mut children = vec![];
    let mut task_split: Vec<[&str; 2]> = vec![];
    for i in 0..lines {
        let begin_index = i * step;
        let mut end_index = (i + 1) * step;
        if end_index > lines {
            end_index = lines;
        }
        let item_check = &chunked_data[begin_index..end_index];
        let mut check_item: [&str; 2] = [item_check[0], item_check[1]];
        task_split.push(check_item);
        if end_index == lines {
            break;
        }
    }
    for (index, line) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", index, line);
        children.push(thread::spawn(move || {
            let result: u32 = line
                .chars()
                .map(|c| c.to_digit(10).expect("invalid digit"))
                .sum();
            println!("line {} result is {}", index, result);
            result
        }))
    }
    let mut intermediate_sums = vec![];
    for child in children {
        let iter_sum = child.join().unwrap();
        intermediate_sums.push(iter_sum);
    }
    let final_sums = intermediate_sums.into_iter().sum::<u32>();
    println!("final sum is {}", final_sums);
}
