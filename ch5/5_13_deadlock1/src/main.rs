// 교착 상태가 발생하는 원인 예제
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // 뮤텍스 2개 생성
    let lock_a = Arc::new(Mutex::new(0));
    let lock_b = Arc::new(Mutex::new(0));

    let lock_a_ref = lock_a.clone();
    let lock_b_ref = lock_b.clone();

    // 스레드 생성하고 b는 잠겨 있는 상태로 만든다
    let thread1 = thread::spawn(move || {
        let a = lock_a.lock().unwrap();
        let b = lock_b_ref.lock().unwrap(); // lock_b는 thread2에 의해 잠겨있는 상태
    });

    // 스레드2 생성하고 a는 잠겨 있는 상태로 만든다.
    let thread2 = thread::spawn(move || {
        let b = lock_b.lock().unwrap();
        let a = lock_a_ref.lock().unwrap(); // lock_a는 thread1에 의해 잠겨있는 상태
    });

    // 스레드가 끝날 때까지 기다린다.
    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("프로그램 종료");
}
