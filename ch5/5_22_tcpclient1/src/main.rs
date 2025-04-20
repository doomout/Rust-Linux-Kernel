// TCP 클라이언트 예제
use std::io::{Read, Write}; //스트림에서 데이터를 읽고 쓰기 위해 import
use std::net::TcpStream; //TCP 클라이언트용 스트림. 서버와의 연결에 사용

fn main() {
    //로컬 호스트(127.0.0.1)의 1234번 포트로 TCP 연결 시도
    //unwrap()은 연결 실패 시 패닉
    let mut stream = TcpStream::connect("127.0.0.1:1234").unwrap();

    let hello = String::from("안녕! 서버!");
    let bytes = hello.as_bytes(); // 문자열을 바이트 배열로
    let len = bytes.len();  // 바이트 길이 계산

    let size_bytes = len.to_ne_bytes();  // 길이를 네이티브 바이트 순서로 변환
    let size_bytes_len = size_bytes.len();
    
    stream.write_all(&len.to_ne_bytes()).unwrap(); // 먼저 길이를 전송
    stream.write_all(&bytes); // 그 다음 실제 데이터 전송
    stream.flush(); // 버퍼에 남은 데이터를 모두 밀어냄

    let mut len_buffer = [0u8; 8];
    stream.read_exact(&mut len_buffer).unwrap(); // 먼저 8바이트 길이 수신

    //서버도 마찬가지로 먼저 길이를 보냄 (8바이트)
    let recv_len = i64::from_ne_bytes(len_buffer).try_into().unwrap();

    //받은 바이트 배열을 i64로 변환 후, usize로 변환 (벡터 크기용)
    let mut txt_buffer = vec![0u8; recv_len];
    stream.read_exact(&mut txt_buffer).unwrap();  // 응답 데이터 읽기
     
    let str = String::from_utf8(txt_buffer.to_vec()).unwrap();
    println!("서버: {:?}", str);
}
/* 핵심 포인트 요약
TCP는 데이터의 경계가 없음 ⇒ 길이 정보를 먼저 보내는 프로토콜 구조가 필요

to_ne_bytes() / from_ne_bytes()를 써서 숫자를 안전하게 바이트로 주고받음

서버도 비슷한 방식으로 구현되어야 제대로 통신 가능함 
*/