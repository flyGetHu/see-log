#[test]
fn test1() {
    let resp = reqwest::blocking::get("https://httpbin.org/ip").unwrap();
    println!("{:#?}", resp);
}
