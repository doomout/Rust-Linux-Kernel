use std::thread;
use std::sync::Mutex;

//Mutex 는 여러 스레드가 공유자원에 동시에 접근하지 못하도록 막는 기법
//Mutex 는 잠금(lock)와 해제(unlock)의 두 가지 상태가 존재
static counter: Mutex<i32> = Mutex::new(0); // counter를 전역변수로 정의

fn inc_counter() {
    // lock을 걸고 접근 권한을 획득함
    //unwrap()은 lock 실패 시 패닉을 발생시킴.
    let mut num = counter.lock().unwrap(); 
    *num = *num + 1; // 자원에 접근하려면 *키워드를 사용한다.
} // inc_counter를 벗어나는 순간 counter는 unlock됩니다.

fn main() {
    let mut thread_vec = vec![];

    // _ 는 변수 이름이 정의되어야 할 부분에서 변수명을 생략할 때 사용한다.
    for _ in 0..100 {
        let th = thread::spawn(inc_counter);
        thread_vec.push(th);
    }

    //각 스레드가 끝날 때까지 기다림.
    for th in thread_vec {
        //join()을 하지 않으면 메인 함수가 먼저 끝날 수도 있음.
        th.join().unwrap();
    }

    // 마지막으로 counter의 값을 lock으로 가져와 출력.
    println!("결과: {}", *counter.lock().unwrap());
}

/*실행결과
결과: 100 
*/