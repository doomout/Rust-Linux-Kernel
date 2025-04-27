// 간단한 명령어 쉘 예제
// 사용자 입력 -> 명령어 분리 -> OS 명령어 실행 -> 결과 출력

use std::io::{self, Write}; // 표준 입출력 모듈 사용
use std::process::Command;  // 외부 명령어 실행을 위한 Command 모듈 사용

fn main() {
    loop { // 무한 루프 - 사용자가 'exit'를 입력할 때까지 반복
        print!("RustShell$ "); // 사용자 프롬프트 출력
        io::stdout().flush().unwrap(); // 버퍼를 비워서 바로 출력되게 함

        let mut input = String::new(); // 사용자 입력을 저장할 문자열 생성
        io::stdin().read_line(&mut input).expect("Failed to read line"); // 사용자 입력을 읽어옴
        let input = input.trim(); // 입력 양 끝의 공백, 개행 문자 제거

        // 사용자가 'exit'를 입력하면 루프를 종료
        if input == "exit" {
            break;
        }

        // 입력을 공백 기준으로 분리하여 parts 벡터에 저장
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue; // 아무것도 입력하지 않았다면 다시 프롬프트 출력
        }

        // parts의 첫 번째 요소를 명령어로 사용
        let command = parts[0];
        // 두 번째 요소부터는 명령어의 인자로 사용
        let args = &parts[1..];

        // 명령어와 인자들을 사용해서 외부 프로그램 실행
        let output = Command::new(command)
            .args(args) // 인자 전달
            .output()   // 명령 실행하고 결과를 얻음
            .expect("Failed to execute command"); // 실행 실패 시 에러 발생

        // 명령어의 실행 결과(stdout)를 출력
        io::stdout().write_all(&output.stdout).unwrap();
        // 명령어 실행 중 에러(stderr)도 출력
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
