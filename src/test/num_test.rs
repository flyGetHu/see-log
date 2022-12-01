use num::Num;

#[test]
fn num_test() {
    let file_lines_count = 1024;
    let max_res_count = 100;
    let difference: isize = file_lines_count - max_res_count;
    let begin_read_index = num::abs(difference);
    println!("{begin_read_index}");
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[test]
fn test2() {
    let num = Number::from(30);
    println!("number:{:?}", num);
    let num2: Number = num.into();
    println!("number:{:?}", num2);
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[test]
fn test3() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}
