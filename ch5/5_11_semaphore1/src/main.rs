/*
세마포어를 사용하려면  tokio 크리이트 사용해야 함
[dependencies]
tokio = { version = "1.25.0", features = ["full"] }
*/

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

/*실행결과
결과: 100
 */