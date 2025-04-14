// LinkedList 예제
use std::collections::LinkedList; // LinkedList 타입을 사용하기 위해 표준 라이브러리 사용

fn main() {
    // i32 타입의 빈 연결 리스트를 생성
    let mut list: LinkedList<i32> = LinkedList::new();

    // 연결 리스트에 값 추가
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // 리스트의 각 요소에 대해 반복하여 출력
    for i in &list {
        print!("{}, ", i);
    }
}

/*실행결과
1, 2, 3,  
*/