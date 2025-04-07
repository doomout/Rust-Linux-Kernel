// 자료를 tail 뒤에 추가하는 연결 리스트
use std::rc::Rc;
use std::cell::RefCell;

struct Person {
    name: String,
    age: i32,
    next: RefCell<Option<Rc<Person>>>,
}

// tail 뒤에 새로운 Person 노드를 추가하는 함수
fn push_back(tail: Rc<Person>, name: String, age: i32) -> Rc<Person> {
    // 새로운 Person 노드를 생성
    // name와 age는 인자로 주어지며, next 필드는 None으로 설정
    let p = Rc::new(Person {
        name: name,
        age: age,
        next: RefCell::new(None) // 새 노드는 다음 노드가 없으므로 None으로 설정
    });

    let mut next = tail.next.borrow_mut();
    *next = Some(p.clone());

    p
}

fn main() {
    let mut head = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: RefCell::new(None),
    }); // Luna, 30, None

    let tail = push_back(head.clone(), String::from("Rust"), 10); // Luna, 30, Some(Rust)
    let tail = push_back(tail.clone(), String::from("Wikibooks"), 20); // Rust, 10, Some(Wikibooks)

    let mut current = head.clone();
    loop {
        print!("{} -> ", current.name);
        let t = current.clone(); // 다음 노드를 참조하기 위해 현재 노드를 복제
        current = match &(*(t.next.borrow_mut())) { //다음 노드
            Some(p) => p,
            None => break,
        }.clone();
    }
}

/*실행결과
Luna -> Rust -> Wikibooks -> 
*/