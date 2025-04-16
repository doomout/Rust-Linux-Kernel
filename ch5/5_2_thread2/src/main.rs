// 스레드를 사용해 파일 읽는 예제
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread;

fn main() {
    // 새로운 스레드를 생성하고, 그 핸들을 받기
    let handle = thread::spawn(|| { 
        //"file.txt" 파일 열기
        let file = File::open("file.txt").unwrap(); 

        // 버퍼링을 사용해 파일 읽기
        let reader = BufReader::new(file); 

        // 파일의 각 줄을 읽어오기
        for line in reader.lines() {
            // 각 줄의 텍스트 읽기
            let txt = line.unwrap();
            println!("{}", txt);
        }
    });

    // 스레드가 끝날 때까지 대기
    // 스레드가 종료되면 join() 메서드가 호출됨
    match handle.join() {
        Ok(_) => {},
        Err(e) => {
            println!("스레드 내부에서 오류가 발생했습니다. {:?}", e);
        }
    };
}
/* 
PowerShell에서는 기본적으로 UTF-16LE (BOM 포함) 으로 저장하기 때문에, 
BufReader로 읽을 때 UTF-8이 아니라서 오류가 발생하기에 아래와 같은 명령어로 파일 생성해야 함
"Hello World" | Out-File -Encoding utf8 file.txt
*/

/*실행결과
1. 파일이 있을 때
Hello World  

2. 파일이 없을 때
thread '<unnamed>' panicked at src\main.rs:10:43:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "지정된 파일을 찾을 수 없습니다." }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
스레드 내부에서 오류가 발생했습니다. Any { .. }
*/