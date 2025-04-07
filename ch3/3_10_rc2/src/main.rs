// RC 로 연결 리스트 만들기
use std::rc::Rc;

struct Person {
    name: String, //사람의 이름을 나타내는 문자열
    age: i32, //사람의 나이를 나타내는 정수
    next: Option<Rc<Person>>, //다음 사람(노드)를 가리키는 옵션 필드, RC<>로 감싸여 있어서 여러곳에서 소유 가능
}

fn main() {
    // p1 노드 생성
    let p1 = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: None,
    });

    //p2 노드생성, 다음 노드로 p1을 가리킴
    let p2 = Rc::new(Person {
        name: String::from("Rust"),
        age: 28,
        next: Some(p1.clone()), // Rust의 다음 노드를 Luna로 설정
    });

    // p2의 이름 출력
    print!("{} -> ", p2.name);

    // p2의 다음 노드를 출력
    match &p2.next {
        //다음 노드가 있다면
        Some(p) => {
            println!("{}", p.name);  // 그 이름을 출력
        },
        //다음 노드가 없다면
        None => {}, // 아무것도 하지 않음
    };
}

/*실행결과
Rust -> Luna
*/