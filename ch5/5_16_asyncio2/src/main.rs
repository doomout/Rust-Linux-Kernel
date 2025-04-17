// 비동기 방식으로 구현한 이벤트 루프 예제
/* 의존성 추가
[dependencies]
tokio = { version = "1.25.0", features = ["full"] }
*/
use tokio::io::{stdin, BufReader, AsyncBufReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(stdin());
    let mut lines = reader.lines();

    loop { // quit가 입력될때 까지 입력을 받음
        match lines.next_line().await.unwrap() {
            Some(input) => {
                println!("입력: {}", input);
        
                if input == "quit" {
                    break;
                }
            }
            None => {
                break;
            },
        };
    }
}

/*실행결과
test      
입력: test
sss
입력: sss
dddd
입력: dddd
quit
입력: quit 
*/