use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::Error;

use crate::business::user::UserManager;
use crate::business::user::User;

// 사용자 목록을 파일에 저장하는 함수
pub fn save(file_name: String, user_vec: Vec<&User>) -> Result<(), Error> {
    // 파일 생성
    let mut buffer = File::create(file_name).expect("파일을 열 수 없습니다.");
    for u in user_vec.iter() {
        // 사용자 정보를 포메팅
        let f = format!("{} {} {}\n", u.id, u.age, u.name);
        // 포메팅된 문자열을 바이트로 변환하여 파일에 기록
        buffer.write(f.as_str().as_bytes())?;
    }

    Ok(())
}

// 파일에서 사용자 목록을 불러오는 함수
pub fn load(file_name: String) -> Vec<User> {
    //사용자 목록을 담을 벡터
    let mut user_vec : Vec<User> = Vec::new();
    // 파일을 읽어오기
    let txt = fs::read_to_string(file_name).expect("파일을 읽을 수 없습니다.");

    // 파일의 각 줄을 분석
    for ln in txt.split("\n") {
        if ln.len() == 0 {
            break;
        }

        // 공백을 기준으로 문자열을 나누어 벡터에 담기
        let tok: Vec<&str> = ln.split(" ").collect();

        // 분리된 토큰을 사용해 사용자 정보 생성
        user_vec.push(User {
            id: tok[0].parse::<i32>().unwrap(),
            age: tok[1].parse::<i32>().unwrap(),
            name: tok[2].to_string(),
        });
    }

    //사용자 벡터 반환
    user_vec
}