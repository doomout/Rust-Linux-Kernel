// Message 라는 열거형(enum)을 정의
// 열거형은 여러 타입을 묶어서 하나의 타입으로 표현할 수 있음
enum Message {
    Quit,               // 값이 없는 열거형
    List(i32),          // 정수형 값을 가지는 열거형
    Put(String),        // 문자열(String)을 가지는 열거형
    Get(i32),           // 정수형 값을 가지는 열거형
}

// Message 열거형에 메서드 정의
impl Message {
    // execute 라는 메서드 정의 (각 열거형에 따라 다른 동작을 함)
    fn execute(&self) {
        // 열거형 타입별로 처리 방식 분기 (패턴 매칭)
        match self {
            Message::Quit => println!("Quit"),               // Quit 이면 출력
            Message::List(val) => println!("List: {}", val), // List 이면 값 출력
            Message::Put(val) => println!("Put: {}", val),   // Put 이면 문자열 출력
            Message::Get(val) => println!("Get: {}", val),   // Get 이면 값 출력
        }
    }
}

fn main() {
    // Message::Put 열거형 생성 (문자열 저장)
    let m = Message::Put(String::from("/root/"));
    m.execute();   // execute 메서드 실행 -> "Put: /root/" 출력

    // Message::List 열거형 생성 (정수 저장)
    let m = Message::List(33);
    m.execute();   // execute 메서드 실행 -> "List: 33" 출력
}

/*실행결과
Put: /root/
List: 33
 */