// 캡슐화를 위해 pub 키워드를 사용하는 예제
// 러스트는 기본적으로는 객체 지향 언어가 아니다.
// 하지만 객제 지향의 핵심적인 개념을 트레잇과 같은 형태로 객체지향에 가까운 프로그램을 작성할 수 있다.
pub struct Student {
    id: i32, // private 필드
    pub name: String, // public 필드
    pub email: String, // public 필드
}

impl Student {
    // public 생성자
    pub fn new(id: i32, name: String, email: String) -> Student {
        Student { id, name, email }
    }

    // public 메서드
    pub fn get_name(&self) -> &String {
        &self.name
    }

    // private 메서드
    fn set_name(& mut self, name: String) {
        self.name = name.clone();
    }
}

fn main() {
    let student = Student::new(1, String::from("luna"), String::from("luna@email.me"));
    println!("이름: {}", student.get_name());
}

/*실행결과
이름: luna 
*/