//함수 파라미터로 값을 전달하는 경우 소유권도 함께 이전 되기에
// 변수의 재사용이 어렵다.

fn main() {
    let s = String::from("Hello");
    push_str(s); // push_str 함수에 소유권을 전달 
    println!("{}", s); // s를 사용하는 순간 컴파일 오류 발생
}

fn push_str(mut s: String) {
    s.push_str(" Rust!");
}

/* 실행 결과
error: aborting due to 1 previous error
*/