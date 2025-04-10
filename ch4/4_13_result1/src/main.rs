// 복구 가능한 오류 예제
use std::fs::File;

fn main() {
    //"test.txt" 이라는 파일을 열려고 시도
    let result = File::open("test.txt");

    //result는 Result 타입으로, 이를 통해 파일 열기의 성공 또는 실패를 확인 가능
    let f = match result { 
        Ok(f) => f, // 파일 열기에 성공시 File 객체를 f에 저장
        Err(err) => { // 파일 열기에 실패하면 에러 정보를 출력하고 프로그램 종료
            panic!("파일 열기 실패: {:?}", err)
        },
    };

    //여기에 도달하면 파일 열기에 성공 했음을 의미
    println!("파일 열기 성공");
}

/*실행결과 
-test.txt 가 있을 때
파일 열기 성공

-test.txt 가 없을 때
파일 열기 실패: Os { code: 2, kind: NotFound, message: "지정된 파일을 찾을 수 없습니다." }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace 
*/