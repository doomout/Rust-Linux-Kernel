// AsMut 는 객체를 수정 가능할 참조로 바꾸는 트레잇이다.
// AsMut 트레잇을 구현한 타입은 &mut T로 변환할 수 있다.
struct Person {
    name: String,
    age: u32,
}

// AsMut 트레잇을 구현하여 name 필드에 대한 가변 참조를 제공한다.
// AsMut 트레잇을 구현하면, &mut T로 변환할 수 있다.
impl AsMut<String> for Person {
    fn as_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

// name_change 함수는 AsMut 트레잇을 구현한 타입에 대해
// name 필드의 값을 변경하는 기능을 제공한다.
// 이 함수는 person 매개변수로 전달된 객체의 name 필드를
// 가변 참조로 가져와서 새로운 이름으로 변경한다.
fn name_change<P: AsMut<String>>(person: &mut P, new_name: &str) {
    let name = person.as_mut();
    name.clear();
    name.push_str(new_name);
}

fn main() {
    let mut person = Person { 
        name: String::from("루나"), 
        age: 10 
    };

    println!("변경 전: {}", person.name); //변경 전: 루나
    name_change(&mut person, "러스트");
    println!("변경 후: {}", person.name); //변경 후: 러스트
}

/*실행 결과
변경 전: 루나
변경 후: 러스트 
*/