// copy trait 은 적용하고자 하는 자료형에 추가하여 사용
#[derive(Copy, Clone)]
struct Student {
    age: i32,
}

fn main() {
    let mut s1 = Student { age: 10 };
    let s2 = s1; // s1을 복사하여 s2에 저장

    println!("s1: {} s2: {}", s1.age, s2.age); //s1: 10 s2: 10

    s1.age = 20; // s1의 나이를 20으로 변경

    println!("s1: {} s2: {}", s1.age, s2.age); //s1: 20 s2: 10
}

/*실행결과 
s1: 10 s2: 10
s1: 20 s2: 10
*/