use std::collections::HashMap;

fn main() {    
    // HashMap 생성
    let mut books: HashMap<i32, String> = HashMap::new();

    // HashMap에 값 추가
    books.insert(10, String::from("Rust"));
    books.insert(20, String::from("Java"));
    books.insert(30, String::from("Python"));

    // get() 메서드를 사용하여 키에 해당하는 값을 가져옴
    // get() 메서드는 Option<T>를 반환하므로 unwrap() 메서드를 사용하여 값을 가져옴
    // get() 메서드는 키에 해당하는 값이 없으면 None을 반환
    let rust = books.get(&10);
    println!("key 10은 {:?}", rust);
}

/*실행결과
key 10은 Some("Rust")
*/