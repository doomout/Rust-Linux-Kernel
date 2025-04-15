// 모듈의 기본 가시성 설정
pub mod business;
pub mod database;

use business::user::UserManager;
use business::user::User;

#[test]
fn it_works() {
    // 테스트를 위한 사용자 추가
    let mut user_mgr = UserManager::new();
    user_mgr.add_user(1, 20, String::from("러스트"));
    user_mgr.add_user(2, 30, String::from("책"));
    
    // 1번 사용자 조회 테스트
    let user = match user_mgr.get_user(1) {
        Some(u) => u,
        _ => {
            panic!("사용자를 찾을 수 없습니다.");
        }
    };

    assert_eq!(user.id, 1); // 1번 사용자 조회

    // 사용자 전체 조회 테스트
    let all_user = user_mgr.get_all();
    for u in all_user.iter() {
        println!("id: {} age: {} name: {}", u.id, u.age, u.name);
    }
    
    // 사용자 삭제 테스트
    println!("1번 러스트 삭제");
    user_mgr.remove_user(1);

    // 삭제 후 1번 사용자 조회 테스트
    match user_mgr.get_user(1) {
        Some(u) => {
            panic!("삭제가 실패했습니다.");
        },
        _ => ()
    };

    // 삭제 후 사용자 전체 조회 테스트
    let all_user = user_mgr.get_all();
    for u in all_user.iter() {
        println!("id: {} age: {} name: {}", u.id, u.age, u.name);
    }
}

// 파일 DB 테스트
// 파일 DB를 사용하여 사용자 정보를 저장하고 불러오는 테스트
#[test]
fn test_filedb() {
    // 테스트를 위한 사용자 추가
    let mut user_mgr = UserManager::new();
    user_mgr.add_user(1, 20, String::from("러스트"));
    user_mgr.add_user(2, 30, String::from("책"));
    
    // 사용자 전체 조회 테스트
    user_mgr.save();

    // 사용자 정보를 파일에 저장
    let mut new_user_mgr = UserManager::new();
    // 파일에서 사용자 정보를 불러오기
    new_user_mgr.load();

    // 1번 사용자 조회 테스트
    // 파일에서 불러온 사용자 정보로 1번 사용자 조회
    // 1번 사용자가 존재하는지 확인
    
    
    let user = match new_user_mgr.get_user(1) {
        // 존재하면 1번 사용자 정보 출력
        Some(u) => u,

        // 존재하지 않으면 panic
        _ => {
            panic!("사용자를 찾을 수 없습니다.");
        }
    };

    // 1번 사용자 정보 확인
    assert_eq!(user.id, 1);

    // 사용자 전체 조회 테스트
    let all_user = new_user_mgr.get_all();
    // 모든 사용자 정보 출력
    for u in all_user.iter() {
        println!("id: {} age: {} name: {}", u.id, u.age, u.name);
    }
}

/*실행결과
running 2 tests
test it_works ... ok
test test_filedb ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests filedb

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s 
*/