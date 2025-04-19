// 데이터 버퍼링 예제
use std::fs::File;
use std::io::{BufRead, BufReader};

/*input.txt 내용
hello world
i am ferris!
haha...
*/

fn main() {
    let file = File::open("input.txt").unwrap();
    // BufReader 생성
    let reader = BufReader::new(file);

    // File 을 읽는다.
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

/*실행결과
hello world
i am ferris!
haha...  
 */