/* 
인코딩을 변경하기 위해서는 Cargo.toml에 아래와 같이 추가한다. 
[dependencies]
encoding_rs = "*"
*/
use encoding_rs::{EUC_KR, UTF_8};
use std::str;

fn main() {
    let utf8_string = "안녕하세요";
    
    // UTF-8로 인코딩된 바이트로 변환
    let utf8_bytes = utf8_string.as_bytes();

    // EUC-KR로 인코딩
    let (euc_kr_bytes, _, _) = EUC_KR.encode(utf8_string);
    
    // 결과 출력
    println!("UTF-8: {:?}", utf8_bytes);
    println!("EUC-KR: {:?}", euc_kr_bytes);
    
    // EUC-KR 바이트 배열을 UTF-8 문자열로 디코딩
    let (utf8_string, _, _) = EUC_KR.decode(&euc_kr_bytes);

    // 결과 출력
    println!("EUC-KR to UTF-8: {}", utf8_string);
}

/*실행결과
UTF-8: [236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148]
EUC-KR: [190, 200, 179, 231, 199, 207, 188, 188, 191, 228]
EUC-KR to UTF-8: 안녕하세요 
*/