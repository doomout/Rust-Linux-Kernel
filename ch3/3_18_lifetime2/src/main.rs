// 런타임 시점에 판단하여 빌림을 반환하는 케이스
// 라이프타임 지시지 `를 사용하여 변수 대여 기간을 명시적으로 지정
// 라이프타임 지시자 `'a`는 'a라는 이름을 가진 라이프타임을 의미
// 'a는 어떤 라이프타임을 나타내는 것이 아니라, 'a라는 이름을 가진 라이프타임을 나타냄
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 'a는 x와 y의 라이프타임을 나타냄
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");

    let result = longest(&s1, &s2);
    println!("{}와 {}중 더 긴 문자열은 '{}'", s1, s2, result);
}

/*실행결과
Hello와 Rust중 더 긴 문자열은 'Hello' 
*/