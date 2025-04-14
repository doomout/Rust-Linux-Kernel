use std::collections::LinkedList;

fn main() {
    // list 이름으로 i32 타입의 LinkedList를 생성
    let mut list: LinkedList<i32> = LinkedList::new();
    // 0부터 9까지의 숫자를 LinkedList에 추가
    // push_back() 메서드를 사용하여 LinkedList에 값을 추가
    for i in 0..10 {
        list.push_back(i);
    }

    // nth() 메서드를 사용하여 9번째 인덱스의 값을 찾음
    let d = list.iter().nth(9);
    println!("target: {:?}", d);
}

/*실행결과
target: Some(9) 
*/