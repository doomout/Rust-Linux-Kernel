// 이진 힙(우선순위 큐) 예제
use std::collections::BinaryHeap;

fn main() {
    // BinaryHeap 생성
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    // BinaryHeap에 값 추가
    heap.push(3);
    heap.push(9);
    heap.push(2);
    heap.push(5);

    // is_empty() 메서드를 사용하여 BinaryHeap에 값이 있는지 확인
    while heap.is_empty() == false {
        //힙의 최대 값을  꺼내서 출력, pop()는 Option<T>를 반환해서 {:?}를 사용해서 반환
        print!("{:?}, ", heap.pop()); 
    }
}

/*실행결과
Some(9), Some(5), Some(3), Some(2), 
*/
