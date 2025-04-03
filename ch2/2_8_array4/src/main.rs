//Rust는 메모리 안정성을 보장하기 위해 배열 범위를 벗어난 인덱스 접근을 허용하지 않는다.

use std::io; // std::io 패키지 로드

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1"); // RUST_BACKTRACE 활성화

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // i32타입을 가지는 5개 원소

    println!("숫자를 입력해주세요.");
    let mut read = String::new(); // 입력값을 저장할 문자열 데이터 생성
    io::stdin().read_line(&mut read).unwrap(); // 키보드 입력을 읽습니다.
    let index: i32 = read.trim().parse().unwrap(); // 문자열을 숫자로 변환합니다.

    println!("arr[{}]={}", index, arr[index as usize]);
}
/** 실행결과
 일부러 인덱스 범위를 벗어난 값을 입력했다 (7 입력) 
 index out of bounds: the len is 5 but the index is 7
 */