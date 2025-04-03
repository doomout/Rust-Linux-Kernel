//자료형 범위를 초과하면 프로그램 실행을 멈춘다.
// Rust는 기본적으로 오버플로우를 체크한다.
fn main() {
    let mut a: i8 = i8::MAX;
    a = a + 1; // MAX 초과
    println!("a = {}", a);    
}

/*실행결과
thread 'main' panicked at main.rs:3:9:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/