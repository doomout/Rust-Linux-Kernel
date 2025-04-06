//구조체 선언
struct Node {
    value: i32, //노드의 값을 저장하는 i32 타입의 필드
    next: Option<Box<Node>>, //다음 노드를 가리키는 필드. Option 를 사용해 노드가 없을 수 있는 상황을 처리
}

// 구조체에 대한 메서드 정의
impl Node {
    fn append(&mut self, elem: i32) {
        match self.next {
            // 값이 있으면~
            Some(ref mut next) => {
                next.append(elem); //마지막 노드를 찾기 위해 다음 노드로 이동
            }
            // 값이 없으면~
            None => {
                let node = Node { // 마지막 노드에 값을 삽입
                    // 새로운 노드를 생성하고
                    value:  elem,
                    // 다음 노드는 None으로 초기화
                    // 즉, 다음 노드가 없음을 나타냄
                    next: None,
                };
                // self.next에 새로운 노드를 박스화하여 저장
                // Box는 힙에 데이터를 저장하고, 포인터를 통해 접근할 수 있게 해줌
                // 즉, self.next는 새로운 노드를 가리키게 됨
                self.next = Some(Box::new(node))
            }
        }
    }

    fn list(&self) {
        print!("{},", self.value);
        match self.next {
            Some(ref next) => next.list(),
            None => {}
        }
    }
}

fn main() {
    // head 노드 생성
    // head 노드는 1로 초기화
    // 즉, head 노드의 값은 1이고, 다음 노드는 None으로 초기화
    let mut head = Node {
        value: 1,
        next: None,
    };

    // 2부터 10까지의 숫자를 head 노드에 추가
    // 즉, head 노드에 2부터 10까지의 숫자를 연결
    // head 노드에 2부터 10까지의 숫자를 추가
    for i in 2..10 {
        head.append(i);
    }

    // head 노드의 값을 출력
    head.list();
}

/*실행결과
1,2,3,4,5,6,7,8,9,
*/