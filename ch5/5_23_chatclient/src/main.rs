use std::io::{self, Write}; // 표준 입력/출력 사용
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader}; // 비동기 읽기/쓰기 및 버퍼 사용
use tokio::net::TcpStream; // TCP 스트림 사용

#[tokio::main] // 비동기 런타임의 진입점
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut username = String::new(); // 사용자 이름을 저장할 변수

    let stream = TcpStream::connect("localhost:20000").await?; // 서버와의 연결을 시도
    let (reader, mut writer) = tokio::io::split(stream); // 읽기와 쓰기 스트림으로 분리
    let mut reader = BufReader::new(reader); // 버퍼를 이용해 읽기 작업 최적화

    print!("대화명을 입력하세요: "); // 사용자에게 대화명 입력 요청
    io::stdout().flush()?; // 출력 버퍼를 즉시 플러시
    io::stdin().read_line(&mut username)?; // 사용자 입력 받기
    writer.write_all(username.as_bytes()).await?; // 서버에 대화명 전송

    // 서버로부터 메시지를 수신하는 비동기 작업
    tokio::spawn(async move {
        loop {
            let mut message = String::new();

            match reader.read_line(&mut message).await { // 서버로부터 메시지 읽기
                Ok(_) => {
                    print!("{}", message); // 메시지 출력
                },
                Err(_) => { // 오류 발생 시 루프 종료
                    break;
                }
            };
        }
    });

    // 사용자가 메시지를 입력하는 메인 루프
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?; // 사용자로부터 메시지 입력 받기
        writer.write_all(input.as_bytes()).await?; // 서버로 메시지 전송

        if input.trim() == "/exit" { // "/exit" 입력 시 종료
            break;
        }
    }

    Ok(()) // 프로그램 종료
}

/*실행결과
대화명을 입력하세요: hi
hi 님이 입장하셨습니다.
안녕~?
hi: 안녕~?
뭐여?
hi: 뭐여?
/exit */