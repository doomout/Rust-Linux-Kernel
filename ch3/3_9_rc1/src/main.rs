// 동적 메모리 할당: RC (Reference Counted)
// Rust는 기본적으로 소유권을 통해 메모리를 관리하지만,
// 때때로 소유권을 공유해야 할 때가 있습니다.
// 이럴 때는 Rc<T>를 사용합니다.
use std::rc::Rc;

struct Person {
    age: i32,
}

fn main() {
    // person을 공유 객체로 생성
    let person = Rc::new(Person { age: 10 });

    // person복제
    let p1 = person.clone(); 
    println!("person: {} p1: {}", person.age, p1.age);
    println!("RefCount: {}", Rc::strong_count(&person)); //+1
    
    // person복제
    let p2 = person.clone();
    println!("RefCount: {}", Rc::strong_count(&person)); //+1

    { //새로운 스코프 시작
        // person복제
        let p3 = person.clone();
        println!("RefCount: {}", Rc::strong_count(&person)); //+1
    } //스코프 종료 (그래서 p3는 소멸됨)

    println!("RefCount: {}", Rc::strong_count(&person)); //-1
}

/*실행결과
person: 10 p1: 10
RefCount: 2
RefCount: 3
RefCount: 4
RefCount: 3
*/