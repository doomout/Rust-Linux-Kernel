/*
Cargo.toml 에 의존성 추가
[dependencies]
tokio = { version = "1.25.0", features = ["full"] }
 */

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