// ? 키워드는 오류를 전파할 때 사용
use std::io;
use std::io::Read;
use std::fs::File;

fn read_from_file(path: String) -> Result<String, io::Error> {
    let mut s = String::new(); // 읽은 문자열을 저장할 문자열 객체
    let mut f = File::open(path)?; // ? 연산자를 사용하여 파일 열기, 실패하면 즉시 오류 반환
    f.read_to_string(&mut s)?; //파일 읽기, 실패하면 즉시 오류 반환
    Ok(s) // 파일 읽기 성공시 문자열 반환
}

fn main() {
    let ret = read_from_file(String::from("test.txt")).expect("파일이 없습니다.");
    println!("test.txt: {}", ret);
}
/* 실행결과
- test.txt 가 있을 때
test.txt: hello

- test.txt 가 없을 때
파일이 없습니다.: Os { code: 2, kind: NotFound, message: "지정된 파일을 찾을 수 없습니다." }
*/