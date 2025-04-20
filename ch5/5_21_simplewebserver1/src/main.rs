//간단한 웹서버 만들기 예제
/*의존성 추가
[dependencies]
tokio = { version = "1.25.0", features = ["full"] }
hyper = { version = "0.14", features = ["full"] }
 */
use hyper::service::{make_service_fn, service_fn}; // 클라이언트 요청마다 서비스를 생성하는 도우미 함수들
use hyper::{Body, Method, Request, Response, Server, StatusCode}; // HTTP 요청/응답에 필요한 타입들

// 에러 처리를 위한 타입 정의
type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

// 클라이언트의 요청(Request)에 따라 적절한 응답(Response)을 반환하는 비동기 함수
async fn response_examples(req: Request<Body>) -> Result<Response<Body>> {
    let index_html = String::from("<h1>Hello World!</h>"); // 기본 페이지 내용
    let notfound_html = String::from("<h1>404 not found</h>"); // 없는 페이지 요청 시 출력할 내용

    // HTTP 메서드(GET)와 요청 경로(/)에 따라 응답 처리
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(index_html.into())), // GET / 요청이면 index 페이지 반환
        _ => {
            // 그 외의 요청은 404 Not Found 응답
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(notfound_html.into())
                .unwrap())
        }
    }
}

// 비동기 main 함수로 서버 구동
#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:20000".parse().unwrap(); // 바인딩할 IP 주소와 포트

    // 클라이언트 연결이 생길 때마다 서비스 인스턴스를 생성하는 함수
    let new_service = make_service_fn(move |_| {
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                response_examples(req) // 요청이 오면 response_examples 함수로 처리
            }))
        }
    });

    // 서버를 해당 주소에서 실행
    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr); // 서버 실행 로그 출력
    server.await?; // 서버 실행을 대기
    Ok(())
}

/* 실행결과
127.0.0.1:20000/ 에 접속시  Hello World! 출력
그 외 에는 404 not found 출력
*/
