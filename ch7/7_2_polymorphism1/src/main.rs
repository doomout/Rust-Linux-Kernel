// 다형성 : 객체가 문맥에 따라 다른 자료형으로 형태를 취할 수 있게 하는 것.
// 러스트는 trait 키워드와 dyn 키워드를 사용하여 다형성을 제공한다.

// trait 정의
trait Hello {
    fn hello_msg(&self) -> String;
}

// 구조체 정의
struct Student {
    name: String,
}

//trait은 "impl 트레잇 이름 for 구조체 이름" 형식으로 정의 한다.
// Hello 트레잇을 Student 구조체에 구현
impl Hello for Student {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요! 선생님,")
    }
}

// 구조체 정의
struct Teacher {
    name: String,
}

// Hello 트레잇을 Teacher 구조체에 구현
impl Hello for Teacher {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요. 오늘 수업은...")
    }
}

fn say_hello(say: &dyn Hello) {
    println!("{}", say.hello_msg());
}

fn main() {
    let student = Student { name: String::from("luna") };
    let teacher = Teacher { name: String::from("me") };

    say_hello(&student); // Student 구조체의 hello_msg 메서드 호출
    say_hello(&teacher); // Teacher 구조체의 hello_msg 메서드 호출
}

/*실행결과
안녕하세요! 선생님,
안녕하세요. 오늘 수업은... 
*/