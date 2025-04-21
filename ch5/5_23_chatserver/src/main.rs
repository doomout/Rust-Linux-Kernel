// 채팅 서비스 서버 부분
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 비동기 TCP 서버를 localhost:20000 포트에서 바인딩합니다.
    let listener = TcpListener::bind("localhost:20000").await?;

    // broadcast 채널 생성 (tx: 송신자, _: 수신자)
    // 크기는 10이며, 연결된 클라이언트들에게 메시지를 브로드캐스트하기 위함입니다.
    let (tx, _) = broadcast::channel(10);

    // 송신자를 Arc로 감싸서 여러 클라이언트 스레드에서 안전하게 공유할 수 있도록 합니다.
    let shared_tx = Arc::new(tx);

    loop {
        // 클라이언트 접속을 기다리며, 접속이 오면 stream을 받아옵니다.
        let (stream, _) = listener.accept().await?;
        let shared_tx = shared_tx.clone(); // Arc는 clone으로 참조를 복사함
        let mut rx = shared_tx.subscribe(); // 새 수신자 생성

        // 클라이언트 한 명에 대해 별도 작업 실행
        tokio::spawn(async move {
            // stream을 읽기와 쓰기로 나눕니다.
            let (reader, mut writer) = tokio::io::split(stream);

            // 다른 유저가 보낸 메시지를 수신해서 현재 유저에게 출력
            tokio::spawn(async move {
                loop {
                    // broadcast로부터 메시지 수신
                    let data: String = match rx.recv().await {
                        Ok(data) => data,
                        Err(_) => return, // 실패하면 종료
                    };

                    if data == "/exit" {
                        break; // 종료 명령
                    }

                    print!("{}", data); // 서버 콘솔 출력 (선택사항)
                    match writer.write_all(data.as_bytes()).await {
                        Ok(_) => {},
                        Err(err) => {
                            println!("네트워크 오류: {:?}", err);
                            return;
                        }
                    };
                }
            });

            // 클라이언트에서 보내는 메시지를 받기 위한 BufReader 생성
            let mut buf_reader = BufReader::new(reader);
            let mut username = String::new();

            // 첫 줄에 유저 이름이 들어온다고 가정
            buf_reader.read_line(&mut username).await;
            let username = username.trim();

            // 입장 메시지를 전 클라이언트에게 전송
            match shared_tx.send(format!("{} 님이 입장하셨습니다.\n", username)) {
                Ok(_) => {},
                Err(_) => return,
            }

            loop {
                let mut message = String::new();
                // 클라이언트로부터 메시지 수신
                buf_reader.read_line(&mut message).await;

                let mut message = String::from(message.trim());
                if message != "/exit" {
                    // 사용자 이름을 포함하여 메시지를 포맷팅
                    message = format!("{}: {}\n", username, message);
                }

                // 모든 클라이언트에게 메시지 브로드캐스트
                match shared_tx.send(message) {
                    Ok(_) => {},
                    Err(_) => break,
                };
            }

            // 클라이언트 종료 메시지 브로드캐스트
            match shared_tx.send(format!("{} 님이 채팅방을 나갔습니다.\n", username)) {
                Ok(_) => {},
                Err(_) => return,
            }
        });
    }
}
