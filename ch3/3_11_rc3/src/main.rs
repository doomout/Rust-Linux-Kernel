// 새로운 노드를 head 앞에 추가하는 예제
use std::rc::Rc;

struct Person {
    name: String, //사람의 이름을 나타내는 문자열
    age: i32, //사람의 나이를 나타내는 정수
    next: Option<Rc<Person>>, //다음 사람(노드)를 가리키는 옵션 필드, RC<>로 감싸여 있어서 여러곳에서 소유 가능
}

//새로운 노드를 head 앞에 추가하는 함수
// head: 현재 리스트의 첫 번째 노드
// name: 새 노드의 이름
// age: 새 노드의 나이
// 반환값: 새 노드의 참조
// 이 함수는 새로운 노드를 생성하고, 그 노드의 next 필드에 현재 리스트의 첫 번째 노드를 연결합니다.
// 그리고 새 노드를 반환합니다.
fn push_front(head: Rc<Person>, name: String, age: i32) -> Rc<Person> {
    // 새로운 노드를 생성합니다.
    // name 과 age는 함수의 인자로 주어지며, next 필드는 기존 연결 리스트의 head을 가리킵니다.
    let p = Rc::new(Person {
        name: name,
        age: age,
        next: Some(head.clone()) // 기존 head를 clone 해 새 노드의 next 로 설정
    });

    p.clone() // 새로 생성된 노드의 rc를 클론해 반환, 이제 이 노드가 새로운 head가 된다.
}

fn main() {
    // head 노드 생성
    // head 노드는 연결 리스트의 첫 번째 노드로, 초기에는 next가 None입니다.
    let head = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: None,
    }); //Luna, 30, None

    let head = push_front(head, String::from("Rust"), 10); // Rust, 10, Some(Luna)
    let head = push_front(head, String::from("Wikibooks"), 20); // Wikibooks, 20, Some(Rust)

    // 연결 리스트를 출력합니다.
    // head 노드부터 시작하여, 각 노드를 순회하며 이름을 출력합니다.
    let mut current = head.clone();

    // current가 None이 아닐 때까지 반복합니다.
    loop {
        print!("{} -> ", current.name);

        // current의 next 필드를 확인합니다.
        // next 필드가 Some인 경우, 그 값을 current에 할당합니다.
        // next 필드가 None인 경우, 반복을 종료합니다.
        current = match &current.next {
            Some(p) => p,
            None => break,
        }.clone();
    }
}

/*실행결과
Wikibooks -> Rust -> Luna -> 
*/