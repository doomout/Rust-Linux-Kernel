// 채널을 사용해 1~100 까지의 합을 전달하는 예제
use std::thread;
use std::sync::mpsc;// mpsc: multiple producer single consumer 의 약자(송신자는 복수, 수신자는 단수)

fn main() {
    // mpsc 채널 생성(tx: 송신자, rx: 수신자)
    let (tx, rx) = mpsc::channel();

    // move : tx 변수를 캡처해 소유권을 해당 스레드를 가지도록 하는 키워드
    thread::spawn(move || {
        let mut sum = 0;

        for i in 1..=100 {
            sum = sum + i;
        }

        // 계산된 합을 채널로 보냄
        tx.send(sum).unwrap();
    });

    // 채널에서 메시지를 수신
    let sum = rx.recv().unwrap();
    println!("1부터 100까지의 합: {}", sum);
}

/*실행결과
1부터 100까지의 합: 5050 
*/