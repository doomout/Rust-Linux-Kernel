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
## 12. 오류처리
rust 는 복구 가능한 오류, 복구 불가능한 오류로 구분하여 예외 처리 한다.
### 복구 가능한 오류
```rust
fn main() {
    //"test.txt" 이라는 파일을 열려고 시도
    let result = File::open("test.txt");

    //result는 Result 타입으로, 이를 통해 파일 열기의 성공 또는 실패를 확인 가능
    let f = match result { 
        Ok(f) => f, // 파일 열기에 성공시 File 객체를 f에 저장
        Err(err) => { // 파일 열기에 실패하면 에러 정보를 출력하고 프로그램 종료
            panic!("파일 열기 실패: {:?}", err)
        },
    };

    //여기에 도달하면 파일 열기에 성공 했음을 의미
    println!("파일 열기 성공");
}
```
- 위의 예제를 축약(unwrap () 사용)
```rust
fn main() {
    // "test.txt"라는 파일을 열려고 시도하며, 실패시 패닉을 일으킨다.
    // unwrap() 메서드는 Result 가 ok값이면 그 값을 반환하고, 
    // Err값이면 panic을 발생시킨다.
    let f = File::open("test.txt").unwrap();
    
    // 여기에 도달하면 파일 열기에 성공했음을 의미한다.
    println!("파일 열기 성공");
}
```
- 지정된 오류 메세지 출력(expect(에러 메세지)사용)
```rust
fn main() {
    // expect() 메서드는 Result가 ok값이면 지정된 에러 메세지를 출력한다.
    let f = File::open("test.txt").expect("에러");
    println!("파일 열기 성공");
}
```
- ? 키워드로 오류 전파
```rust
fn read_from_file(path: String) -> Result<String, io::Error> {
    let mut s = String::new(); // 읽은 문자열을 저장할 문자열 객체
    let mut f = File::open(path)?; // ? 연산자를 사용하여 파일 열기, 실패하면 즉시 오류 반환
    f.read_to_string(&mut s)?; //파일 읽기, 실패하면 즉시 오류 반환
    Ok(s) // 파일 읽기 성공시 문자열 반환
}

fn main() {
    let ret = read_from_file(String::from("test.txt")).expect("파일이 없습니다.");
    println!("test.txt: {}", ret);
}
```
### 복구 불가능한 오류
```rust
fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        // panic! 는 복구 불가능한 오류를 일으키려고 할 때 쓰인다.
        panic!("0으로 나눌 수 없습니다.")
    }

    a / b
}

fn main() {
    let ret = div(1, 0);
    println!("ret: {}", ret);
}
```
## 13. 컬렉션
### 컬렉션 종류
- Sequence
    - Vec: 크기 조정이 가능한 배열
    - VecDeque: 앞뒤 양쪽에 자료를 추가, 삭제할 수 있는 큐
    - LinkedList: 한 줄로 연결되게 관리하는 자료구조, 삽입/삭제가 빈번할 때 사용
- Map
    - HashMap: Key, Value의 쌍으로 이루어진 자료구조
    - BTreeMap: B-Tree를 기반으로 하는 정렬된 Map
- Set
    - HashSet: 하나의 자료만 담을 수 있는 집합 구조
    - BTreeSet: B-Tree를 기반으로 하는 정렬된 Set
- 기타
    - BinaryHeap: 우선순위 큐
    - String: 문자열
### Vee : 인덱스로 바로 접근 가능한 가변 배열
```rust
// 벡터 생성
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];

// 요소 추가
v.push(4);

// 요소 접근
let third = v[2];
match v.get(2) {
    Some(x) => println!("세 번째 요소: {}", x),
    None => println!("없음"),
}

// 요소 변경
v[1] = 20;

// 요소 삭제
v.remove(0); // 0번 인덱스 삭제
v.pop();     // 마지막 요소 삭제
```
### LinkedList :  요소 삽입/삭제가 많고 중간 삽입이 필요한 경우 적합.
```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();

// 요소 추가
list.push_back(10); // 뒤에 추가
list.push_front(5); // 앞에 추가

// 요소 삭제
list.pop_back();
list.pop_front();

// 반복 및 접근
for val in &list {
    println!("{}", val);
}
```
### HashMap : 키-값 쌍 저장용.
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// 값 추가
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 20);

// 값 접근
if let Some(score) = scores.get("Blue") {
    println!("Blue 팀 점수: {}", score);
}

// 반복
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 값 갱신
scores.insert(String::from("Blue"), 30);

// 키가 없을 때만 삽입
scores.entry(String::from("Green")).or_insert(50);
```
### HashSet :  중복 없는 값 저장용.
```rust
use std::collections::HashSet;

let mut set = HashSet::new();

// 값 추가
set.insert("apple");
set.insert("banana");

// 포함 여부 확인
if set.contains("apple") {
    println!("사과 있음!");
}

// 값 제거
set.remove("banana");

// 반복
for item in &set {
    println!("{}", item);
}
```
### Binary Heap : 최대, 최소값을 찾을 때 많이 사용
```rust
//최대값
let mut heap = BinaryHeap::new();
// 값 추가 (자동 정렬됨 - 최대 힙)
heap.push(10);
heap.push(5);
heap.push(20);

// 최대값 확인 (pop 없이)
if let Some(max) = heap.peek() {
    println!("최대값: {}", max); // 20
}

// 최대값 꺼내기 (pop은 값을 제거함)
if let Some(max) = heap.pop() {
    println!("꺼낸 값: {}", max); // 20
}

// 반복적으로 pop하면 큰 값부터 나옴
while let Some(val) = heap.pop() {
    println!("{}", val);
}

//최소 값
let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(10));
min_heap.push(Reverse(1));
min_heap.push(Reverse(5));

while let Some(Reverse(val)) = min_heap.pop() {
    println!("{}", val); // 1, 5, 10 순으로 출력
}
```
### String : 문자열은 벡터를 사용해 문자를 관리.
```rust
// String은 UTF-8로 인코딩된 문자열을 저장한다.
fn main() {
    // String은 String::new()로 생성할 수 있다.
    let mut eng = String::new();
    // String은 push_str() 메서드를 사용하여 문자열을 추가할 수 있다.
    eng.push_str("hello");

    // to_string() 메서드를 사용하여 &str을 String으로 변환할 수 있다.
    let jpn = "こんにちは".to_string();
    // String::from() 함수를 사용하여 &str을 String으로 변환할 수 있다.
    let kor = String::from("안녕하세요");

    println!("{} {} {}", eng, jpn, kor);
}
// format! 매크로로 포매팅된 문자열 만들수 있다.
fn main() {
    let str = String::from("안녕");
    let idx = 123;

    // format! 매크로는 문자열을 포매팅하여 새로운 String을 생성한다.
    // format! 매크로는 {}를 사용하여 포매팅할 위치를 지정한다.
    let s = format!("{} {}", str, idx);
    println!("{}", s);
}
// 문자열 내 문자들을 탐색하기
fn main() {
    let txt = String::from("안녕하세요.");

    // txt 내의 문자들을 순회한다.
    for c in txt.chars() {
        print!("{} ", c);
    }
}
```
## 14. 동시성
### thread : 스레드를 생성하고 제어
```rust
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread;

fn main() {
    // 새로운 스레드를 생성하고, 그 핸들을 받기
    let handle = thread::spawn(|| { 
        //"file.txt" 파일 열기
        let file = File::open("file.txt").unwrap(); 

        // 버퍼링을 사용해 파일 읽기
        let reader = BufReader::new(file); 

        // 파일의 각 줄을 읽어오기
        for line in reader.lines() {
            // 각 줄의 텍스트 읽기
            let txt = line.unwrap();
            println!("{}", txt);
        }
    });

    // 스레드가 끝날 때까지 대기
    // 스레드가 종료되면 join() 메서드가 호출됨
    match handle.join() {
        Ok(_) => {},
        Err(e) => {
            println!("스레드 내부에서 오류가 발생했습니다. {:?}", e);
        }
    };
}
```
### mpsc : 채널을 생성하여 여러 스레드의 데이터를 공유
```rust
fn main() {
    //mpsc 채널 생성 tx는 송신자, rx는 수신자
    let (tx1, rx) = mpsc::channel(); 
    let tx2 = mpsc::Sender::clone(&tx1); // tx1복제

    // 1부터 50까지의 합
    thread::spawn(move || {
        let mut sum = 0;

        for i in 1..=50 {
            sum = sum + i;
        }

        tx1.send(sum).unwrap();
    });

    // 51부터 100까지의 합
    thread::spawn(move || {
        let mut sum = 0;

        for i in 51..=100 {
            sum = sum + i;
        }

        tx2.send(sum).unwrap();
    });

    let mut sum = 0;
    
    for val in rx {
        println!("수신: {}", val);
        sum = sum + val;
    } 

    println!("1부터 100까지의 합: {}", sum);
}
```
### async/await : 비동기 프로그래밍을 할 수 있는 기법 제공
async/await 를 사용하려면 Cargo.toml 파일에 추가
[dependencies]
futures = "0.3"
```rust
// await 를 사용해 다른 async 함수 호출 예제
use futures::executor::block_on;

//비동기 함수 정의
async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        sum += i;
    }

    sum
}
//비동기 함수 정의
async fn calc() -> i32 {
    let sum1_50 = calc_sum(1, 50).await; //await 키워드롤 결과 얻기
    let sum51_100 = calc_sum(51, 100).await; //await 키워드롤 결과 얻기
    let ret = sum1_50 + sum51_100;

    ret
}

fn main() {
    let future = calc();

    // block_on() 실행 calc가 종료될 때까지 메인 스레드는 멈춘다.
    let sum = block_on(future); 
    println!("1부터 100까지의 합: {}", sum);
}
```
### tokio : 비동기(Async) 작업을 실행할 수 있도록 도와주는 런타임 라이브러리
Cargo.toml 에 의존성 추가
[dependencies]
tokio = { version = "1.25.0", features = ["full"] }
```rust
use std::time::Duration;
use tokio::time;

// async 로 비동기 함수로 지정
async fn sleep_10sec() {
    for i in 1..10 {
        println!(".");
        time::sleep(Duration::from_millis(1000)).await; // 1초간 10회 대기
    }
}

// async 로 비동기 함수로 지정
async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        println!("{} ", i);
        sum += i;
    }

    sum
}

async fn calc() -> i32 {
    let f1 = sleep_10sec();
    let f2 = calc_sum(1, 10);

    // sleep_10sec() 와 calc_sum()가 동시에 수행된다.
    let (_, sum) = tokio::join!(f1, f2); //tokio::join!() 를 사용해 비동기 함수를 대기한다.
    sum
}

//tokio를 사용하는 비동기 메인 함수
#[tokio::main]
async fn main() {
    let sum = calc().await;

    println!("sum={}", sum);
}
```
## 15. 이벤트 루프 모델과 스레드 모델의 장단점
### 이벤트 루프
- 특징: 단일 스레드를 사용, 이벤트를 종료될 때까지 무한 반복해서 처리
- 장점: 최소한의 오베헤드로 많은 이벤트 처리 가능, 복잡한 동기화 필요 없음
- 단점: 단일 스레드를 사용하여 CPU 연산이 많은 작업에는 부적합
- 사용 예: UI가 있는 프로그램, I/O 작업이 많은 서비스 등
### 스레드
- 특징: 스레드별로 작업이 할당되는 방식.
- 장점: CPU 코어가 충분할 경우 복수의 작업을 병렬화해 빠르게 전체 작업을 수행
- 단점: CPU 코어 갯수가 넘어설 정도로 많은 처리를 할시 스위칭 비용으로 성능 지연 발생, 복잡한 동기화 메커니즘 필요
- 사용 예: CPU 연산량이 많은 작업(알고리즘 등)
## 16. 동시성 제어 기법
### 뮤텍스: 여러 스레드가 공유 자원에 동시에 접근하지 못하도록 막는 기법
```rust
use std::thread;
use std::sync::Mutex;

static counter: Mutex<i32> = Mutex::new(0); // 전역 뮤텍스 변수

fn inc_counter() {
    let mut num = counter.lock().unwrap(); // 뮤텍스 잠금
    *num += 1; // 값 증가
} // 함수 종료 시 자동으로 unlock

fn main() {
    let mut thread_vec = vec![];

    for _ in 0..100 {
        let th = thread::spawn(inc_counter); // 스레드 생성
        thread_vec.push(th); // 핸들 저장
    }

    for th in thread_vec {
        th.join().unwrap(); // 스레드 종료 대기
    }

    println!("결과: {}", *counter.lock().unwrap()); // 최종 결과 출력
}
```
### 세마포어: 복수의 제한된 자원에 다수의 스레드가 동시에 접근하는 것을 막는 제어 방법
```rust
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;

// 공유 카운터를 위한 뮤텍스
static counter: Mutex<i32> = Mutex::new(0);

#[tokio::main]
async fn main() {
    // 동시에 2개의 thread가 접근 가능하도록 세마포어 설정
    let semaphore = Arc::new(Semaphore::new(2));
    let mut future_vec = vec![];

    for _ in 0..100 {
        // semaphore 획득
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let future = tokio::spawn(async move {
            let mut num = counter.lock().unwrap(); //뮤텍스로부터 안전한 참조 획득
            *num = *num + 1; //카운터 증가

            drop(permit); // semaphore 해제
        });
        future_vec.push(future); //생성된 future를 벡터에 저장
    }

    for future in future_vec {
        future.await.unwrap(); //모든 future가 완료될 때까지 대기
    }

    println!("결과: {}", *counter.lock().unwrap()); //최종 결과 출력
}
```
## 17. Copy 와 Clone 비교
| 구분       | Copy 트레잇                 | Clone 트레잇                         |
|------------|-----------------------------|--------------------------------------|
| 복사 방식  | 묵시적 복사                  | 명시적 복사 (`clone()` 호출 필요)     |
| Drop 트레잇과의 관계 | 함께 사용 불가               | 함께 사용 가능                         |
| 사용 대상  | 단순한 자료 구조             | 복잡한 자료 구조 등                    |
| 로직 구현  | 불가능                      | 가능                                  |

## 18. AsRef 와 AsMut 비교
| 항목         | AsRef<T>                         | AsMut<T>                          |
|--------------|----------------------------------|-----------------------------------|
| 목적         | 참조를 불변으로 변환              | 참조를 가변으로 변환               |
| 사용 시기    | 읽기 전용으로 변환할 때           | 값을 수정할 수 있도록 변환할 때    |
| 반환 타입    | `&T`                             | `&mut T`                          |
| 주요 특징    | 읽기 전용 접근 제공              | 수정 가능 접근 제공               |
| 구현 예시    | `impl AsRef<str> for String`     | `impl AsMut<Vec<u8>> for MyType` |

## 19. Path 와 PathBuf 비교
| 구분	    | Path	                 | PathBuf                         |
|-----------|-----------------------|----------------------------------|
| 역할	    | 불변 경로 참조	     | 가변 경로 소유 객체              |
| 가변성	| 변경 불가 (immutable)	 | 경로를 조작할 수 있음 (mutable)  |
| 소유권	| 소유하지 않음 (참조형) | 경로 자체를 소유함                |
| 생성 방식	| Path::new("경로")	     | PathBuf::from("경로")            |
| 사용 예시	| 경로 정보 확인, 비교	  | 경로 확장, 조작, 파일명 추가 등  |

## 20. 객제지향 러스트
- 캡슐화 : 객체의 내부 정보를 숨겨 외부에서 접근할 수 없게 제어 하는 것
    - pub : 외부에 노출할 부분과 그렇지 않은 부분은 분리
- 다형성 : 객체가 문맥에 따라 다른 자료형으로 형태를 취할 수 있게 하는 것
    - trait : 인터페이스와 같은 개념
    - dyn : trait 을 참조할 때 사용, 어떠한 타입도 참조 가능
    - impl : 구현을 의미 trait 에서 정의한 기능을 실제 구조체에서 구현할 때 사용
- 상속 : 러스트에는 상속이 없다. 하지만 유사하게 구현 한다.
    - trait 를 사용하여 인터페이스를 구현하고, 구조체 내부에 다른 구조체를 포함하여 기능을 재사용한다.

## 21. 디자인 패턴
- 디자인 패턴 범주
    - 생성: 객체의 생성과 관련된 패턴 (빌더, 팩토리 메서드, 싱글턴 등)
    - 구조: 객체 간의 구조화와 관련된 패턴(어댑터, 브리지, 컴포지트, 데코레이터, 플라이웨이트 등)
    - 행동: 알고리즘이나 객체 간 작동에 관현된 패턴(반복자, 옵저버, 상태, 전략)
- 팩토리 메서드 패턴
    - 객체의 생성을 별도의 생성 모듈에 위임하여 객체의 생성 과정과 관리를 생성 모듈이 제어하는 패턴
- 싱글턴 패턴
    - 시스템에 하나의 인스턴스만 생성되도록 하는 디자인 패턴