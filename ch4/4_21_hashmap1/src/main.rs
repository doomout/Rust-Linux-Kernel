// 해시 맵 예제
use std::collections::HashMap;

fn main() {    
    // HashMap 생성
    let mut books: HashMap<i32, String> = HashMap::new();

    // HashMap에 값 추가
    // insert() 메서드를 사용하여 키와 값을 추가
    // 키는 i32, 값은 String 타입
    books.insert(10, String::from("Rust"));
    books.insert(20, String::from("Java"));
    books.insert(30, String::from("Python"));

    // HashMap 출력
    // for 루프를 사용하여 HashMap의 모든 키와 값을 출력
    for (key, value) in &books {
        println!("Key: {:?}, Value: {:?}", key, value);
    }
}

/*실행결과
Key: 30, Value: "Python"
Key: 20, Value: "Java"
Key: 10, Value: "Rust" 
*/