// RefCell<T>를 사용해 next 의 참조값을 변경하는 예제
use std::rc::Rc;
use std::cell::RefCell; //내부 가변성을 제공

struct Person {
    name: String,
    age: i32,
    next: RefCell<Option<Rc<Person>>>, // RefCell<>로 감싸서 next는 수정 가능
}

fn main() {
    // p1 노드 생성
    let mut p1 = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: RefCell::new(None), //처음에는 다음 노드가 없음.
    });

    // p2 노드 생성
    let mut p2 = Rc::new(Person {
        name: String::from("Rust"),
        age: 10,
        next: RefCell::new(None), //처음에는 다음 노드가 없음.
    });

    // p1의 next 필드에 대한 가변 참조를 얻음
    let mut next = p1.next.borrow_mut();

    // * 키워드를 사용하면 RefCell<T>의 내부 값을 얻을 수 있다.
    *next = Some(p2.clone()); // p1뒤에 p2를 추가
}
  