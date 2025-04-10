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
## 3. 조건문
```rust
fn main() {
    let number = 10;

    // if 조건문
    if number > 5 {
        println!("큰 숫자!");
    } else {
        println!("작은 숫자!");
    }

    // let-if 문
    let condition = true;
    // if문의 값을 ret에 저장
    let ret = if condition == true {
        String::from("조건이 참 입니다.") // ;을 붙이면 컴파일 오류 발생
    } else {
        String::from("조건이 거짓 입니다.") // ;을 붙이면 컴파일 오류 발생
    }; // 여기는 ;을 붙여야 한다.
    
    println!("ret={}", ret);
    
    // match 문
    let var = 1;
    match var {
        1 => println!("하나"),
        2 => println!("둘"),
        _ => println!("기타"), // default 조건
    }

    // let-match 문
    let var = 1;
    let ret = match var {
        1 => String::from("하나"),
        2 => String::from("둘"),
        _ => String::from("기타"),
    }; // 세미콜론을 붙여야 합니다.
    
    println!("ret={}", ret);
}
```
## 4. 반복문
```rust
fn main() {
//loop 반복문
    loop {
        println!("숫자를 입력해주세요. 0을 입력하면 종료합니다");
        let mut read = String::new();
        io::stdin().read_line(&mut read).unwrap();
        let val: i32 = read.trim().parse().unwrap();

        if val == 0 {
            break; //종료
        }

        println!("입력={}", val);
    }

// for 문: 전체 반복
    let arr = [1, 2, 3, 4, 5];
    for a in arr { //arr 전체를 순회
        print!("{}, ", a);
    }
// for 문: 범위로 반복
    for a in 0..5 { //5회 반복
        print!("{},", a);
    }

// while 문
    let mut counter = 0;
    while counter < 5 { //counter 가 5보다 작을 때까지 반복
        print!("{},", counter);
        counter += 1;
    }
    
}
```
## 5. 함수 및 반환값
```rust
//반환 값이 없는
fn add(x: i32, y: i32) {
    println!("{}+{}={}", x, y, (x + y));
}

//반환 값이 있는 함수
fn add(a: i32, b: i32) -> i32 { //-> i32는 반환 값의 타입을 나타냅니다.
    a + b // 세미콜론 없음 = 반환 값
}

//익명 함수
let x = 1;
let y = 2;
let ret = { // 익명 함수의 반환값을 ret에 저장
    x + y //;이 없다.
}; //여기에 ;이 있다.
println!("{}+{}={}", x, y, ret)

// 클로저 함수
fn main() {
    let x = 10; // 변수 x 선언 (정수 10)

    // 클로저 함수 add 정의
    let add = |y| x + y; // x 값을 내부에서 사용함

    println!("10 + 5 = {}", add(5)); // 클로저 호출 (5) : 10 + 5가 됨
}
```
## 6. 구조체, 메서드, 연관 함수
- struct 가 데이터고
- impl 은 그 데이터가 할 수 있는 동작(method)을 구현하는 공간
```rust
// 데이터(필드)만 정의
struct Score {
    score: i32,
}

// Score에 관련된 동작(메서드) 정의
// impl: 구조체(struct)나 enum에 동작(method)을 구현(implement)할 때 사용
impl Score { 
    // &self 붙으면 → 인스턴스 메서드
    // self 없음 → 정적 메서드
    fn get_grade(&self) -> String {
        if self.score >= 90 {
            String::from("A")
        } else if self.score >= 80 {
            String::from("B")
        } else {
            String::from("C")
        }
    }
    // 정적 메서드
    fn from(score: i32) -> Score {
        //Score {필드이름: 값}
        Score { score: score }
    }
}

```
## 7. 열거형 (Enum), Option 열거형
- 러스트는 null 이 없다.
```rust
// Message 라는 열거형(enum)을 정의
// 열거형은 여러 타입을 묶어서 하나의 타입으로 표현할 수 있음
enum Message {
    Quit,               // 값이 없는 열거형
    List(i32),          // 정수형 값을 가지는 열거형
    Put(String),        // 문자열(String)을 가지는 열거형
    Get(i32),           // 정수형 값을 가지는 열거형
}

// Message 열거형에 메서드 정의
impl Message {
    // execute 라는 메서드 정의 (각 열거형에 따라 다른 동작을 함)
    fn execute(&self) {
        // 열거형 타입별로 처리 방식 분기 (패턴 매칭)
        match self {
            Message::Quit => println!("Quit"),               // Quit 이면 출력
            Message::List(val) => println!("List: {}", val), // List 이면 값 출력
            Message::Put(val) => println!("Put: {}", val),   // Put 이면 문자열 출력
            Message::Get(val) => println!("Get: {}", val),   // Get 이면 값 출력
        }
    }
}
// Option<T> 는 null 대신 사용하는 열거형 타입
fn print_optional(val: Option<String>) {
    // match 구문을 사용해서 Option 값을 패턴 매칭으로 처리
    match val {
        // 값이 있을 때 (Some) → 내부 값 출력
        Some(val) => println!("{}", val),
        // 값이 없을 때 (None) → "None" 출력
        None => println!("None"),
    }
}
```
##  8. 소유권 (Ownership) 및 참조
- 소유권의 장점
    - 메모리 안전성: 강력하고 엄격한 소유권 검사기를 통해 메모리 안전성 보장
    - 성능: 쓰레기 수집기의 오베헤드 없이 빠르고 효율적으로 메모리 관리
    - 스레드 안전성: 소유권은 하나의 값을 하나의 스레드만 접근 가능하도록 한정
- 소유권의 단점
    - 학습 비용: 러스트만의 독창적인 개념으로 익숙치 않은 개발자는 학습 시간 걸림
    - 코드 구조 난해: 소유권을 만족시키기 위해 코드 구조가 장황하고 복잡해짐
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
## 9. 패턴 매칭 (Match)
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
## 10. Result 및 Option
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
## 11. 정보 제한자
- pub: 정보를 제한 없이 외부에 공개
- pub(in 대상 모듈): 제공된 경로에 한정해 정보를 제공
- pub(crate): 현재 crate에 한정해 정보를 제공
- pub(super): 상위 모듈에 정보를 제공
- pub(self) 또는 생략: 정보를 외부에 공개하지 않음