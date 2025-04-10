use std::fs::File;

fn main() {
    // expect() 메서드는 Result가 ok값이면 지정된 에러 메세지를 출력한다.
    let f = File::open("test.txt").expect("에러");
    println!("파일 열기 성공");
}

/*실행결과
- test.txt 가 있을 때
파일 열기 성공

- test.txt 가 없을 때
thread 'main' panicked at src\main.rs:4:36:
에러: Os { code: 2, kind: NotFound, message: "지정된 파일을 찾을 수 없습니다." }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\result2.exe` (exit code: 101)
*/