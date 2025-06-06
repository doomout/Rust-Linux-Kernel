// Rust의 함수 정의
// Rust에서는 함수를 정의할 때 fn 키워드를 사용합니다.
// 함수 이름은 소문자로 시작하며, 여러 단어로 이루어진 경우에는 snake_case를 사용합니다.
fn main() {
    add(1, 2);
}

// 숫자 두 개를 더하는 함수
fn add(x: i32, y: i32) {
    println!("{}+{}={}", x, y, (x + y));
}

/*실행결과
1+2=3 
*/