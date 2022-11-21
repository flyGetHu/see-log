#[test]
fn num_test() {
    let file_lines_count = 1024;
    let max_res_count = 100;
    let difference: isize = file_lines_count - max_res_count;
    let begin_read_index = num::abs(difference);
    println!("{begin_read_index}")
}
