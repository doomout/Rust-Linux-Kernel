use std::io;

#[derive(Debug)] // 구조체의 디버그 출력
struct Student {
    id: i32,
    name: String,
    email: String,
}

fn create_student(id: i32, name: String, email: String) -> Student {
    Student { // Student 인스턴스 생성
        id: id,
        name: name,
        email: email,
    } // 세미콜론이 없다는데 주의
}

fn main() {
    println!("학번을 입력해주세요.");
    let mut id = String::new();
    io::stdin().read_line(&mut id);
    let id: i32 = id.trim().parse().unwrap();

    println!("이름을 입력해주세요.");
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    let name = name.trim().to_string(); // 공백 제거

    println!("이메일을 입력해주세요.");
    let mut email = String::new();
    io::stdin().read_line(&mut email);
    let email = email.trim().to_string(); // 공백 제거

    let stu = create_student(id, name, email); // trim함수로 공백 제거
    // 기존방법
    // println!("학번={}, 이름={}, 이메일={}", stu.id, stu.name, stu.email);

    // :? 는 디버그 출력을 의미
    println!("stu={:?}", stu);
}

/*실행결과
학번을 입력해주세요.
2025
이름을 입력해주세요.
김재벌
이메일을 입력해주세요.
aaa@gmail.com
stu=Student { id: 2025, name: "김재벌", email: "aaa@gmail.com" }
 */