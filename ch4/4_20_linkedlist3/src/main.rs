// 링크드 리스트 값 변경 예제
use std::collections::LinkedList;

fn main() {
    // 생성하고~
    let mut list: LinkedList<i32> = LinkedList::new();
    // 반복해서 값 넣고~
    for i in 0..10 {
        list.push_back(i);
    }

    // 넣은 값에 +10 씩해서 변경하고~
    for d in list.iter_mut() {
        *d += 10;
    }

    // 변경된 값 출력하고~
    for d in list.iter() {
        print!("{:?}, ", d);
    }
}

/*실행결과
10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 
*/