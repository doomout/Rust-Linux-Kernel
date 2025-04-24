//fs 모듈을 사용하여 파일을 생성하고 읽는 예제
use std::fs::File;
use std::io::{self, Read, Write};

//? 전제 조건
//main() 함수나 해당 위치 함수의 반환 타입이 Result<...> 이어야 한다.그래야 에러를 return할 수 있다.
fn main() -> io::Result<()> {
    //?를 붙이면 에러가 발생하면 즉시 현재 함수를 빠져나가고, 에러를 호출자에게 전달
    let mut file = File::create("example.txt")?; // 파일 생성

    // b"Hello"처럼 b를 붙이면 &[u8] 타입, 즉 바이트 슬라이스가 된다.
    // write_all 메서드는 &[u8] 타입의 데이터를 받아들이기 때문
    file.write_all(b"Hello, Rust!")?; // 파일에 내용 추가

    // 파일 읽기
    let mut file = File::open("example.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{}", content);

    Ok(())
}

/*실행결과
Hello, Rust! 
*/