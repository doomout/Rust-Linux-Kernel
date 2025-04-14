// format! 매크로로 포매팅된 문자열 만들수 있다.
fn main() {
    let str = String::from("안녕");
    let idx = 123;

    // format! 매크로는 문자열을 포매팅하여 새로운 String을 생성한다.
    // format! 매크로는 {}를 사용하여 포매팅할 위치를 지정한다.
    let s = format!("{} {}", str, idx);
    println!("{}", s);
}

/*실행결과
안녕 123
*/