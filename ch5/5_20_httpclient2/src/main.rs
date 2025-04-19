// REST API 사용하기
// REST API 란? HTTP 프로토콜 기반의 API, 웹 서비스 <-> 클라이언트 간 데이터 통신하는 방법
// GET, POST, PUT, DELETE 등으로 외부 자원에 접근

/*의존성 추가
[dependencies]
serde = {version = "1.0", features = ["derive"]}
serde_json = "1.0"
tokio = { version = "1.25.0", features = ["full"] }
hyper = { version = "0.14", features = ["full"] } */

use hyper::body::Buf;
use hyper::Client;
// JSON 파싱을 위한 Deserialize trait 사용
use serde::Deserialize;

// JSON 데이터를 담을 구조체 정의 (역직렬화 대상)
#[derive(Deserialize, Debug)]
struct User {
    id: i32,         // 사용자 ID
    name: String,    // 사용자 이름
}

// tokio의 비동기 런타임에서 main 함수 정의
#[tokio::main]
async fn main() {
    // API URL 문자열을 URL 객체로 파싱
    let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();
    
    // HTTP 클라이언트 생성
    let client = Client::new();

    // GET 요청 보내고 응답을 기다림
    let res = client.get(url).await.unwrap();

    // 응답 body 전체를 메모리에 모아서 가져옴 (aggregate는 비동기적으로 전체 body 수신)
    let body = hyper::body::aggregate(res).await.unwrap();

    // JSON 데이터를 파싱해서 Vec<User>로 변환 (reader는 Buf를 읽을 수 있는 std::io::Read처럼 사용)
    let users: Vec<User> = serde_json::from_reader(body.reader()).unwrap();
    
    // 파싱된 결과를 보기 좋게 출력
    println!("users: {:#?}", users);
}
/*실행결과
users: [
    User {
        id: 1,
        name: "Leanne Graham",
    },
    User {
        id: 2,
        name: "Ervin Howell",
    },
    User {
        id: 3,
        name: "Clementine Bauch",
    },
    User {
        id: 4,
        name: "Patricia Lebsack",
    },
    User {
        id: 5,
        name: "Chelsey Dietrich",
    },
    User {
        id: 6,
        name: "Mrs. Dennis Schulist",
    },
    User {
        id: 7,
        name: "Kurtis Weissnat",
    },
    User {
        id: 8,
        name: "Nicholas Runolfsdottir V",
    },
    User {
        id: 9,
        name: "Glenna Reichert",
    },
    User {
        id: 10,
        name: "Clementina DuBuque",
    },
] */