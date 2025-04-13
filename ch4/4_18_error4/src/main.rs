// 복구 불가능한 오류 발생
fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        // panic! 는 복구 불가능한 오류를 일으키려고 할 때 쓰인다.
        panic!("0으로 나눌 수 없습니다.")
    }

    a / b
}

fn main() {
    let ret = div(1, 0);
    println!("ret: {}", ret);
}

/*실행 결과
thread 'main' panicked at main.rs:5:9:
0으로 나눌 수 없습니다. 
*/