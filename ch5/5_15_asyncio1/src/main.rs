// 비동기식 입출력 예제
// 이 코드는 비동기식으로 처리 해서 메인 스레드를 멈추지 않는다.

/*cargo.toml에 의존성 추가
[dependencies]
tokio = { version = "1.25.0", features = ["full"] } 
*/
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // 비동기 방식으로 file 핸들러를 얻는다.
    //input.txt 엔 hello world 가 입력되어 있다.
    let mut file = File::open("input.txt").await.unwrap(); 
    let mut contents = String::new();

    // 비동기 방식으로 file 읽기
    file.read_to_string(&mut contents).await.unwrap();

    // input.txt의 내용을 출력
    println!("{}", contents);

    //비동기 방식으로 file 생성
    let mut file = File::create("output.txt").await.unwrap();
    //비동기 방식으로 file 저장 (input.txt에 있는 hello world가 output.txt에 저장된다.)
    file.write_all(contents.as_bytes()).await.unwrap();
}

/*실행결과
hello world 
*/