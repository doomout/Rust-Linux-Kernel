// Setter 함수를 자동으로 생성하는 예제
macro_rules! create_accessors { //접근자를 생성하는 매크로
    ($name:ident, $type:ty, $setter:ident) => { //변수명과 타입, setter 함수명을 입력 받는다.
        fn $name(&self) -> &$type {
            &self.$name
        }

        fn $setter(&mut self, value: $type) { //setter 함수를 생성한다.
            self.$name = value;
        }
    };
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    create_accessors!(name, String, set_name);
    create_accessors!(age, u32, set_age);
}

fn main() {
    let mut person = Person { name: "루나".to_string(), age: 10 };
    person.set_name("하이".to_string());
    person.set_age(8);

    println!("이름: {} 나이: {}", person.name, person.age)
}

/*실행결과
이름: 하이 나이: 8
*/