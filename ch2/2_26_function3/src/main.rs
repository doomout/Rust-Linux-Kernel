//익명 함수 예제

fn main() {
    let x = 1;
    let y = 2;
    let ret = { // 익명 함수의 반환값을 ret에 저장
        x + y //;이 없다.
    }; //여기에 ;이 있다.

    println!("{}+{}={}", x, y, ret);
}

/*실행결과
1+2=3 
*/