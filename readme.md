### Rust 문법 요약
## 1. 변수 및 상수
- 러스트는 기본 변수가 불변성을 갖는다.
- 변수의 값을 변경할 경우 컴파일 오류를 일으킨다.
- mut 를 사용하면 변수의 값을 변경할 수 있다.
- 불변성의 장점: 병렬 처리가 용이하다.
- 불변성의 단점: 코드 로직이 복잡해지고 값을 복제하는 과정에서 자원 비효율이 발생한다.
```rust
fn main() {
    let x = 5; // 기본 변수 (불변)
    let mut y = 10; // 변경 가능한 변수
    const PI: f64 = 3.1415; // 상수 (타입 명시 필수)

    println!("x: {}, y: {}, PI: {}", x, y, PI);
}
```
## 2. 데이터 타입
```rust
fn main() {
    let byte_num: i8 = 8; // 8비트 정수(1바이트)
    let short_num: i16 = 16; // 16비트 정수(2바이트)
    let int_num: i32 = 32; // 32비트 정수(4바이트)
    let long_num: i64 = 64; // 64비트 정수(8바이트)
    let float_num: f32 = 3.14; // 32비트 실수(4바이트)
    let double_num: f64 = 3.14; // 64비트 실수(8바이트)
    let is_active: bool = true; // 불리언(1바이트)
    let letter: char = 'R'; // 문자(4바이트)
    let tuple: (i32, f64, char) = (42, 3.14, 'R'); // 튜플(가변)
    let array: [i32; 3] = [1, 2, 3]; // 배열(가변)

    println!("Tuple: {:?}", tuple);
    println!("Array: {:?}", array);
}
```
## 3. 조건문과 반복문
```rust
fn main() {
    let number = 10;

    // 조건문
    if number > 5 {
        println!("큰 숫자!");
    } else {
        println!("작은 숫자!");
    }

    // 반복문
    for i in 1..=5 {
        println!("반복: {}", i);
    }

    let mut count = 0;
    while count < 3 {
        println!("while 반복: {}", count);
        count += 1;
    }
}
```
## 4. 함수 및 반환값
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // 세미콜론 없음 = 반환 값
}

fn main() {
    let sum = add(3, 4);
    println!("3 + 4 = {}", sum);
}
```
## 5. 구조체 (Struct) 및 메서드
```rust
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn greet(&self) {
        println!("안녕하세요! 저는 {}이고, {}살입니다.", self.name, self.age);
    }
}

fn main() {
    let person = Person {
        name: String::from("홍길동"),
        age: 25,
    };

    person.greet();
}
```
## 6. 열거형 (Enum)
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(direction: Direction) {
    match direction {
        Direction::Up => println!("위로 이동!"),
        Direction::Down => println!("아래로 이동!"),
        Direction::Left => println!("왼쪽으로 이동!"),
        Direction::Right => println!("오른쪽으로 이동!"),
    }
}

fn main() {
    let dir = Direction::Left;
    move_player(dir);
}
```
##  7. 소유권 (Ownership) 및 참조
```rust
fn main() {
    let s = String::from("Hello");
    
    takes_ownership(s); // 소유권 이동
    // println!("{}", s); // ❌: 이미 이동됨

    let x = 10;
    makes_copy(x); // 복사 (i32는 Copy 트레이트 적용됨)

    let s2 = String::from("Rust");
    let len = calculate_length(&s2); // 참조 사용
    println!("{}의 길이: {}", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len() // 참조를 사용하여 소유권 이동 없음
}
```
## 8. 패턴 매칭 (Match)
```rust
fn main() {
    let number = 3;

    match number {
        1 => println!("하나"),
        2 => println!("둘"),
        3 => println!("셋"),
        _ => println!("다른 숫자"),
    }
}
```
## 9. Result 및 Option
```rust
fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("0으로 나눌 수 없습니다!"))
    } else {
        Ok(x / y)
    }
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("결과: {}", result),
        Err(e) => println!("에러 발생: {}", e),
    }
}
```