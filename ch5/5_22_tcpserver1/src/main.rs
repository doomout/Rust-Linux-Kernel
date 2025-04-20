// TCP 서버 예제
use std::io::{Read, Write}; // 입출력 관련 기능
use std::net::{TcpListener, TcpStream}; // TCP 연결을 위한 기능

// 클라이언트와 데이터를 주고받는 함수
fn handle_client(mut stream: TcpStream) {
    let mut len_buffer = [0u8; 8]; // 수신 데이터 길이를 받을 버퍼

    // 클라이언트가 보낸 데이터 길이를 받음
    stream.read_exact(&mut len_buffer).unwrap();
    let recv_len = i64::from_ne_bytes(len_buffer).try_into().unwrap(); // 수신한 길이로 데이터 크기 계산

    let mut txt_buffer = vec![0u8; recv_len]; // 실제 데이터를 받을 버퍼
    stream.read_exact(&mut txt_buffer).unwrap(); // 데이터를 읽음
    
    // 수신한 데이터를 문자열로 변환
    let str = String::from_utf8(txt_buffer.to_vec()).unwrap();
    println!("클라이언트: {:?}", str); // 클라이언트 메시지 출력

    // 서버에서 응답할 메시지 준비
    let hello = String::from("안녕! 서버!"); 
    let bytes = hello.as_bytes(); // 바이트로 변환
    let len = bytes.len(); // 메시지 길이 계산
    
    // 길이 정보와 메시지 전송
    stream.write_all(&len.to_ne_bytes()).unwrap(); // 메시지 길이를 전송
    stream.write_all(&bytes); // 실제 메시지를 전송
}

fn main() {
    // 서버가 연결을 기다리는 포트 바인딩
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    // 연결이 들어오면 처리
    for stream in listener.incoming() {
        let stream = stream.unwrap(); // 연결된 클라이언트 스트림 처리
        handle_client(stream); // 클라이언트와 데이터 교환 함수 호출
    }
}
