// String 타입을 사용하여 문자열을 생성하고 수정하는 예제
// String 타입은 가변적이며, 문자열을 동적으로 생성하고 수정할 수 있습니다.
// String 타입은 Rust의 표준 라이브러리에서 제공하는 문자열 타입으로, 가변적이고 동적으로 크기를 조절할 수 있습니다.
// String 타입은 힙에 저장되며, 문자열을 조작하는 다양한 메서드를 제공합니다.
// String 타입은 Rust의 표준 라이브러리에서 제공하는 문자열 타입으로, 가변적이고 동적으로 크기를 조절할 수 있습니다.
// String 타입은 힙에 저장되며, 문자열을 조작하는 다양한 메서드를 제공합니다.
fn main() {
    let mut s = String::from("Hello"); // Hello로 String 생성
    println!("{}", s); 
    s.push_str(" Rust!"); // s뒤에 Rust! 추가
    println!("{}", s);
}

/* 실행결과
  Hello
  Hello Rust!
*/