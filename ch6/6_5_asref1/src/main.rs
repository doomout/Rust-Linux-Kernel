// AsRef 트레잇은 어떤 타입을 다른 타입으로 변환할 수 있는 방법을 제공합니다.
// 이 트레잇을 구현하면, 해당 타입을 다른 타입으로 쉽게 변환할 수 있습니다.
// 예를 들어, String을 &str로 변환하거나, Vec<T>를 &[T]로 변환할 수 있습니다.
// AsRef 트레잇을 구현한 타입은, 해당 타입을 다른 타입으로 변환할 수 있는 메서드를 제공합니다.
// 이 메서드는 AsRef 트레잇을 구현한 타입의 인스턴스에서 호출할 수 있습니다.
struct Person {
    name: String,
    age: u32,
}

impl AsRef<str> for Person {
    // Person의 name을 str형태로 참조할 수 있습니다.
    fn as_ref(&self) -> &str {
        &self.name
    }
}

fn greet_person<P: AsRef<str>>(person: P) {
    println!("안녕! {}!", person.as_ref());
}

fn main() {
    let person = Person { name: String::from("루나"), age: 30 };

    // Person 구조체에 AsRef<str>를 구현했기 때문에, 
    // greet_person 함수는 Person을 인자로 받아 사용할 수 있습니다.
    greet_person(person); //안녕! 루나!

    // 물론, String과 &str도 여전히 함수의 인자로 사용할 수 있습니다.
    let name_string = String::from("러스트");
    greet_person(name_string); //안녕! 러스트!
    greet_person("하이!"); //안녕! 하이!!
}

/*실행결과
안녕! 루나!
안녕! 러스트!
안녕! 하이!! 
*/