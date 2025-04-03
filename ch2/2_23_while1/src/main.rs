
// 조건식을 입력 받을 수 있는 반복문
fn main() {
    let mut counter = 0;

    // while 문은 조건식이 true일 때 반복
    // 조건식이 false가 되면 반복 종료
    while counter < 5 { //counter 가 5보다 작을 때까지 반복
        print!("{},", counter);
        counter += 1;
    }
}

/* 실행결과
0,1,2,3,4,
*/