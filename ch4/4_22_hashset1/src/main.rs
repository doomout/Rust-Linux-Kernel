// HashSet 은 중복을 허용하지 않는 집합 자료구조입니다.
// HashSet은 해시 함수를 사용하여 데이터를 저장하고 검색하는 데 사용됩니다.
// HashSet은 데이터의 순서를 보장하지 않으며, 데이터의 중복을 허용하지 않습니다.
use std::collections::HashSet;

fn main() {
    // HashSet 생성
    let mut book : HashSet<String> = HashSet::new();
    
    book.insert(String::from("Rust")); // HashSet에 값 추가
    book.insert(String::from("Rust")); // 중복이라 이 값은 무시
    book.insert(String::from("Rust")); // 중복이라 이 값은 무시
    book.insert(String::from("Java")); // HashSet에 값 추가 
    
    for data in &book {
        println!("{:?}", data);
    }

    // HashSet에 값이 있는지 확인
    // contains() 메서드를 사용하여 HashSet에 값이 있는지 확인
    if book.contains("Python") == false {
        println!("Python이 없습니다.")
    }
}

/*실행결과
"Java"
"Rust"
Python이 없습니다. 
*/