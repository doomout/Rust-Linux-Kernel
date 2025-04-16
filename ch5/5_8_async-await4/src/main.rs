// 비동기 함수 내에서 스레드 관련 작업을 할 때 발생하는 문제
use std::thread;
use std::time::Duration;
use futures::executor::block_on;

async fn sleep_10sec() {
    for i in 1..10 {
        println!(".");
        thread::sleep(Duration::from_millis(1000)); // 1초간 10회 대기
    }
}

async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        println!("{} ", i);
        sum += i;
    }

    sum
}

async fn calc() -> i32 {
    let f1 = sleep_10sec(); //이게 실행되면 모든 이벤트 루프 스레드가 작동을 멈춘다.
    let f2 = calc_sum(1, 10);
    let (_, sum) = futures::join!(f1, f2); //f1, f2 가 끝나기를 기다린다.

    sum
}

fn main() {
    let future = calc();
    let sum = block_on(future);
    println!("1부터 10까지의 합: {}", sum);
}
/*실행결과
.
.
.
.
.
.
.
.
.
1 
2 
3
4
5
6
7
8
9
10
1부터 10까지의 합: 55 
*/