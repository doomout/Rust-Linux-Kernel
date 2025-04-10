use std::io;
use std::io::Read;
use std::fs::File; // 파일 시스템 작업을 위한 머듈

fn read_from_file(path: String) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e), //파일 열기 실패 시 오류 반환
    };

    match f.read_to_string(&mut s) {
        Ok(_len) => return Ok(s), //파일 읽기 성공시 내용 반환
        Err(e) => return Err(e), //파일 읽기 실패 시 오류 반환
    };
}

fn main() {
    // "test.text" 파일을 읽으려 시도하고,
    // 실패하면 "파일 읽기 중 오류가 발생했습니다."라는 메시지와 함꼐 프로그램 종료
    let ret = read_from_file(String::from("test.txt")).expect("파일 읽기 중 오류가 발생했습니다.");
    println!("test.txt: {}", ret);
}

/*실행결과
- test.txt 가 있을 때
test.txt: hello

- test.txt 가 없을 때
thread 'main' panicked at src\main.rs:21:56:
파일 읽기 중 오류가 발생했습니다.: Os { code: 2, kind: NotFound, message: "지정된 파일을 찾을 수 없습니다." }
*/