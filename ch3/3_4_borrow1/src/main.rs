// 빌림을 사용해 소유권 공유
// 값은 전달하되 소유권은 유지하고 싶을 경우 사용
fn main() {
    let mut s = String::from("Hello");

    //s 의 소유권을 이전하지 않고, 참조를 전달해 문자열의 내용을 변경
    push_str(&mut s);

    // s는 소유권을 유지하고 있기에 정상 동작
    println!("{}", s);
}

fn push_str(s: &mut String) {
    s.push_str(" Rust!");
}

/*실행결과
Hello Rust!
*/