use futures::executor::block_on;

// 비동기 함수 정의
async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        sum += i;
    }

    sum
}

fn main() {
    let future = calc_sum(1, 100);

    // block_on() 실행 calc_sum()가 종료될 때까지 메인 스레드는 멈춘다.
    let sum = block_on(future);
    println!("1부터 100까지의 합: {}", sum);
}
/*실행결과
1부터 100까지의 합: 5050 
*/