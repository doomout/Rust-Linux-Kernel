// 이중 연결 리스트 예제
use std::cell::RefCell;
use std::rc::Rc;

//NodeType을 Otion<Rc<RefCell<Node>>>로 정의, Node의 이전 노드와 다음 노드를 참조하는 타입을 나타낸다.
//RC와 RefCell을 사용하면 노드를 가변으로 공유할 수 있으며, None를 사용하면 마지막 노드를 표현할 수 있다.
type NodeType = Option<Rc<RefCell<Node>>>;

struct Node {
    item: i32, //노드에 저장된 정수값
    prev: NodeType, //이전 노드, 노드가 첫번째 노드인 경우 None
    next: NodeType, //다음 노드, 노드가 마지막 노드인 경우 None
}

impl Node {
    fn new(item: i32) -> Self {
        Self {
            item,
            prev: None,
            next: None,
        }
    }
}

pub struct DoubleLinkedList {
    head: NodeType,
    tail: NodeType,
}

impl DoubleLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    // 뒤에 삽입
    fn push_back(& mut self, item: i32) {
        // 새로운 노드를 생성
        let node = Rc::new(RefCell::new(Node::new(item)));

        // tail이 있다면 맨 뒤에 삽입
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(Rc::clone(&node)); //현재 tail 의 next를 새로운 노드로 설정
            node.borrow_mut().prev = Some(tail); //새로운 노드의 prev를 현재 tail로 설정
            self.tail = Some(node); //새 노드를 새로운 tail로 설정
        } else {
            // tail이 없다면 head와 tail을 모두 새로운 노드로 설정
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    // 앞에 삽입
    fn push_front(&mut self, item: i32) {
        let node = Rc::new(RefCell::new(Node::new(item)));

        // head가 있다면 맨 앞에 삽입
        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Some(Rc::clone(&node));
            node.borrow_mut().next = Some(head);
            self.head = Some(node);
        } else {
            // tail이 없다면 head와 tail을 모두 새로운 노드로 설정
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    // 전체 코드를 순회
    fn print_all(&mut self) {
        let mut current = match &self.head {
            Some(n) => {
                n.clone()
            },
            None => {
                return;
            }
        };

        // 전체를 순회하면서 값을 출력
        loop {
            let t = current.clone();
            println!("item: {}", t.borrow().item);
            current = match &(t.borrow().next) {
                Some(n) => {
                    n
                },
                None => break,
            }.clone();
        }
    }
}

fn main() {
    let mut list = DoubleLinkedList::new();

    println!("뒤에 1,2,3 삽입");
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    list.print_all();
    
    println!("맨 앞에 0 추가");
    list.push_front(0);
    list.print_all();
}

/*실행결과
뒤에 1,2,3 삽입
item: 1
item: 2
item: 3
맨 앞에 0 추가
item: 0
item: 1
item: 2
item: 3 
*/