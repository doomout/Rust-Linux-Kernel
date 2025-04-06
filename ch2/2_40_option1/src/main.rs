// Option<String> 타입의 값을 출력하는 함수 정의
// Option<T> 는 null 대신 사용하는 열거형 타입
fn print_optional(val: Option<String>) {
    // match 구문을 사용해서 Option 값을 패턴 매칭으로 처리
    match val {
        // 값이 있을 때 (Some) → 내부 값 출력
        Some(val) => println!("{}", val),
        // 값이 없을 때 (None) → "None" 출력
        None => println!("None"),
    }
}

fn main() {
    // Some은 값이 있는 상태를 나타냄
    let some_string = Some(String::from("러스트"));

    // None은 값이 없는 상태 (null 개념)
    let none_string: Option<String> = None;

    // 값이 있는 Some("러스트") 출력
    print_optional(some_string);

    // 값이 없는 None 출력
    print_optional(none_string);
}


/*실행결과
러스트
None
*/