/*의존성 추가
[dependencies]
shared_memory = "*"
*/
// 공유 메모리를 읽기 위한 프로그램
extern crate shared_memory;

use shared_memory::*;
use std::time::Duration;
use std::thread;

// 공유 메모리에서 사용할 데이터 구조체
#[repr(C)]
struct SharedData {
    number: i32,
}

fn main() {
    let shmem_flink = "/tmp/basic_mapping.map";

    // 공유메모리 파일 생성
    let shmem = match ShmemConf::new().size(4096).flink(shmem_flink).create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => ShmemConf::new().flink(shmem_flink).open().unwrap(),
        Err(e) => {
            eprintln!("공유 메모리 파일 생성 실패 {shmem_flink} : {e}");
            return;
        }
    };

    // 공유 메모리 데이터 포인터 획득
    let shared_data: &SharedData = unsafe { &*(shmem.as_ptr() as *const SharedData) };

    while shared_data.number < 60 {
        println!("Reading: {}", shared_data.number);
        thread::sleep(Duration::from_secs(1));
    }
}

/*실행결과
Reading: 5
Reading: 5
Reading: 5
Reading: 5
Reading: 5
Reading: 5
Reading: 5
Reading: 5
*/