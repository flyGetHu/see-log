use salvo::http::cookie::time;
use std::thread;
use std::time::Duration;

#[test]
fn workout_test() {
    // 动作次数
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;

    let begin_time = time::Date::MAX;
    let do_something: fn(time::Date) -> i32 = |begin_time| {
        println!("mumumu.....");
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", begin_time);
        10
    };
    if intensity < 25 {
        println!("今天活力满满，先做 {} 个俯卧撑!", do_something(begin_time));
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            do_something(begin_time)
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            do_something(begin_time)
        );
    }
}
