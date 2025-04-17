// 동기식 입출력 방식의 예제
use std::fs::File;
use std::io::{Read, Write};

// 동기식 입출력 방식은 작업이 완료 될 때까지 무한정 기다린다.
// 가장큰 문제는 입출력이 완전히 완료될 때까지 기다리니 CPU 자원을 비효율적으로 사용한다.
fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // 파일을 읽을때 까지 대기합니다.

    println!("{}", contents);

    let mut file = File::create("output.txt").unwrap();
    file.write_all(contents.as_bytes()).unwrap();
    // 파일을 쓸때 까지 대기합니다.
}
