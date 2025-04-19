//SQLite 사용하는 예제
/* SQLite 의존성 추가
[dependencies]
sqlite = "0.30" */
use sqlite;
use sqlite::State;

fn main() {
    // 메모리에 sqlite db 생성
    let connection = sqlite::open(":memory:").unwrap();

    // users 테이블 만들고, 2개 데이터 삽입
    let query = "
        CREATE TABLE users (name TEXT, age INTEGER); 
        INSERT INTO users VALUES ('루나', 3);
        INSERT INTO users VALUES ('러스트', 13);
    ";
    // 테이블 생성 쿼리를 실행
    connection.execute(query).unwrap();

    // ?는 나중에 이 자리에 값을 넣을꺼야 표시
    // SQL 인젝션을 막기 위해 사용
    let query = "SELECT * FROM users WHERE age > ?"; 
    
    // 쿼리를 실행
    let mut statement = connection.prepare(query).unwrap();

    // (1, 5) → 1은 첫 번째 파라미터 위치 (?)를 의미
    // 5는 ?에 들어갈 값
    statement.bind((1, 5)).unwrap(); // age > 5
    
    // 테이블의 데이터를 조회
    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
    }
}

/*실행결과
name = 러스트
age = 13 */