// clone() 메서드는 Clone trait을 구현한 타입에서만 사용할 수 있다.
// Clone trait을 구현한 타입은 clone() 메서드를 사용하여 객체를 복사할 수 있다.
// Clone trait을 구현하지 않은 타입은 clone() 메서드를 사용할 수 없다.
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    cloned: bool,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Person { 
            name: self.name.clone(), 
            age: self.age,
            cloned: true
        }
    }
}

fn main() {
    let person1 = Person {
        name: String::from("루나"),
        age: 10,
        cloned: false
    };

    // person1을 복제합니다. 소유권을 잃지 않습니다.
    let person2 = person1.clone();

    println!("{:?}", person1);
    println!("{:?}", person2);
}

/*실행결과
Person { name: "루나", age: 10, cloned: false }
Person { name: "루나", age: 10, cloned: true } 
*/