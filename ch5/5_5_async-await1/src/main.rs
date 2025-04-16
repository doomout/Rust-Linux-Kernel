use futures::executor::block_on;

// async 키워드 사용해 비동기 함수 정의
async fn hello_world() {
    println!("future안에서 실행");
}

fn main() {
    let future = hello_world(); // 함수가 바로 호출되지 않는다.
    println!("main함수에서 실행");

    //future 를 실행. hello_world가 종료될 때까지 main thread 는 멈춘다.
    block_on(future);
    println!("future종료 이후 실행");
}

/*실행결과
main함수에서 실행
future안에서 실행
future종료 이후 실행 
*/