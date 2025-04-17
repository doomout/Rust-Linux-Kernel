// Arc를 사용해 스레드 간 자원을 공유하는 예제
use std::thread;
use std::sync::{Arc, Mutex};

//Arc<T>란? 동시적 상황에서 사용 가능한 참조 카운팅 스마트 포인터
//Arc<T> 는 여러 스레드에서 동시에 접근하더라도 안전하게 참조 횟수를 관리
fn main() {
    //공유될 카운터를 Arc 와 Mutex로 감싸준다.
    let counter = Arc::new(Mutex::new(0));
    // 스레드를 저장할 벡터
    let mut thread_vec = vec![];

    for _ in 0..100 {
        //현재 카운터의 클론을 생성
        let _cnt = counter.clone();

        let th = thread::spawn(move || {
            // 뮤텍스로부터 안전하게 락을 얻어와 참조를 획득
            let mut num = _cnt.lock().unwrap();
            // 카운터 값을 증가
            *num = *num + 1;
        });
        //각 스레드의 핸들을 벡터에 넣는다.
        thread_vec.push(th);
    }

    for th in thread_vec {
        // 모든 스레드가 끝날 때까지 기다린다.
        th.join().unwrap();
    }

    // 최종 카운터 값을 출력한다.
    println!("결과: {}", *counter.lock().unwrap());
}

/*실행결과
결과: 100 
*/