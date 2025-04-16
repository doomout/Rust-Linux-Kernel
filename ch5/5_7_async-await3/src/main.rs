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
/*실행결과
1부터 100까지의 합: 5050 
*/