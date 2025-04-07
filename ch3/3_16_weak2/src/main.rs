// Rc와 Weak를 사용해서 순환 참조 문제를 해결하는 예제
use std::rc::{Rc, Weak};  // Rc: 강한 참조 / Weak: 약한 참조
use std::cell::RefCell;   // 내부 가변성을 위해 사용 (가변 borrow)

struct Person {
    id: i32,                          // Person 식별자 (id)
    next: RefCell<Option<Weak<Person>>>, // 다음 사람 참조 (약한 참조)
    // Weak<T> 를 사용하면 참조카운트를 증가시키지 않음
}

// Rc 참조카운트가 0이 되면 자동 호출되는 drop 함수 구현
impl Drop for Person {
    fn drop(&mut self) {
        // 메모리 해제될 때 어떤 Person이 drop 되었는지 출력
        println!("p{} Drop!", self.id);
    }
}

fn main() {
    // p1 객체 생성 (참조카운트 strong=1)
    let mut p1 = Rc::new(Person {
        id: 1,
        next: RefCell::new(None),
    });

    // p2 객체 생성 (참조카운트 strong=1)
    let mut p2 = Rc::new(Person {
        id: 2,
        next: RefCell::new(None),
    });

    // p1 -> p2 연결 (약한 참조로 연결)
    {
        let mut next = p1.next.borrow_mut();        // RefCell을 사용해 내부 가변 접근
        *next = Some(Rc::downgrade(&p2));           // Rc::downgrade() : Weak 참조 생성
        // Rc::downgrade를 사용하면 참조카운트 증가하지 않음 (순환참조 해결)
    }

    // p2 -> p1 연결 (약한 참조로 연결)
    {
        let mut next = p2.next.borrow_mut();
        *next = Some(Rc::downgrade(&p1));           // 약한 참조로 연결
    }

    // 현재 Rc 참조카운트 출력
    println!("p1 RefCount: {} p2: RefCount: {}", 
        Rc::strong_count(&p1), Rc::strong_count(&p2));
    // 출력 예시
    // p1 RefCount: 1 p2 RefCount: 1

    // 약한 참조는 strong_count 증가 안되므로
    // main 스코프 종료시 Rc 카운트 1 -> 0 으로 떨어짐
    // -> Drop 호출됨
}

/* 실행결과
p1 RefCount: 1 p2 RefCount: 1
p2 Drop!
p1 Drop!
*/

/*
종류                | 	참조 카운트 증가 여부 |	특징           |	주 용도
Rc<T> (강한 참조)   |	참조카운트 증가	      | 데이터 소유    |	일반적인 참조
Weak<T> (약한 참조) |	참조카운트 증가 안함  |	데이터 소유 X  |	순환참조 방지
*/