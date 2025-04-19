// HTTP 클라이언트 만들기
/*의존성 추가
[dependencies]
tokio = { version = "1.25.0", features = ["full"] }
hyper = { version = "0.14", features = ["full"] } */
use hyper::{body::HttpBody as _, Client};
use tokio::io::{stdout, AsyncWriteExt as _};

#[tokio::main]
async fn main() {
    let client = Client::new();
    // 외부 ip 조회하는 사이트
    let uri = "http://httpbin.org/ip".parse().unwrap();

    // http 요청을 보낸다.
    let mut resp = client.get(uri).await.unwrap();
    println!("Response: {}", resp.status());
    
    // 응답 온 body 값을 확인 한다.
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk.unwrap()).await.unwrap();
    }
}
/*실행결과
Response: 200 OK
{
  "origin": "154.20.54.2"
}
*/  