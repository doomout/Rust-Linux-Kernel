// rc로 선언된 데이터를 수정하려고 시도하는 예제
// rc는 불젼성을 가진 참조형이기에 공유 데이터를 변경할 수 없다.
use std::rc::Rc;

struct Person {
    name: String,
    age: i32,
    next: Option<Rc<Person>>, // next는 수정 불가
}

fn main() {
    let mut p1 = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: None, // next는 수정 불가
    });

    let mut p2 = Rc::new(Person {
        name: String::from("Rust"),
        age: 10,
        next: None,
    });

    p1.next = Some(p2.clone()); // 컴파일 오류 발생
}
 /* 실행결과
error[E0594]: cannot assign to data in an `Rc`
  --> main.rs:24:5
   |
24 |     p1.next = Some(p2.clone()); // 컴파일 오류 발생
   |     ^^^^^^^ cannot assign
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<Person>`

error: aborting due to 1 previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0594`.
  */