// String 는 소유권을 가지고 있으며 크기를 동적으로 변경할 수 있다.
// String은 heap에 저장된다.
// String은 UTF-8로 인코딩된 문자열을 저장한다.
fn main() {
    // String은 String::new()로 생성할 수 있다.
    let mut eng = String::new();
    // String은 push_str() 메서드를 사용하여 문자열을 추가할 수 있다.
    eng.push_str("hello");

    // to_string() 메서드를 사용하여 &str을 String으로 변환할 수 있다.
    let jpn = "こんにちは".to_string();
    // String::from() 함수를 사용하여 &str을 String으로 변환할 수 있다.
    let kor = String::from("안녕하세요");

    println!("{} {} {}", eng, jpn, kor);
}

/*실행결과
hello こんにちは 안녕하세요
*/
