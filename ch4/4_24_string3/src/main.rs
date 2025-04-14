// 문자열 내 문자들을 탐색하기
fn main() {
    let txt = String::from("안녕하세요.");

    // txt 내의 문자들을 순회한다.
    for c in txt.chars() {
        print!("{} ", c);
    }
}
/* 실행결과
안 녕 하 세 요 . 
*/
