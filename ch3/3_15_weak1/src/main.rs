// Rc와 RefCell을 사용한 순환 참조 메모리 누수 예제
use std::rc::Rc;
use std::cell::RefCell;

// Person 구조체 정의
struct Person {
    id: i32,                        // Person 식별자 (id)
    next: RefCell<Option<Rc<Person>>>, // 다음 사람을 가리키는 참조
}

// Rc 참조카운트가 0이 되면 자동 호출되는 drop 함수 구현
impl Drop for Person {
    fn drop(&mut self) {
        // drop이 호출되면 어떤 Person이 해제되는지 출력
        println!("p{} Drop!", self.id);
    }
}

fn main() {
    // p1 객체 생성 (Rc로 감싸서 참조 카운트 관리)
    let mut p1 = Rc::new(Person {
        id: 1,
        next: RefCell::new(None),   // next는 None으로 초기화
    });

    // p2 객체 생성
    let mut p2 = Rc::new(Person {
        id: 2,
        next: RefCell::new(None),
    });

    // p3 객체 생성
    let mut p3 = Rc::new(Person {
        id: 3,
        next: RefCell::new(None),
    });

    {
        let mut next = p1.next.borrow_mut(); // RefCell을 사용해 내부 가변 접근
        *next = Some(p2.clone());           // p1 뒤에 p2 추가
    }

    {
        let mut next = p2.next.borrow_mut();
        *next = Some(p1.clone());           // p2 뒤에 p1 추가
    }

    // p1, p2의 참조 카운트 출력
    println!("p1 RefCount: {} p2 RefCount: {}", 
        Rc::strong_count(&p1), Rc::strong_count(&p2));
    // 출력 예시
    // -> Rc 참조카운트가 0이 되어야 Drop 호출됨
    // -> 하지만 p1 <-> p2가 서로를 참조(순환 참조)하고 있어서
    // Rc 참조카운트가 0이 되지 않음
    // -> Drop 함수 호출 안 됨 (메모리 누수 발생)
ㅠㅒ
    // 반면 p3는 순환 참조 없이 main 스코프 끝나면 Drop 호출됨
}

/*실행결과 
p1 RefCount: 2 p2 RefCount: 2
p3 Drop! 
*/