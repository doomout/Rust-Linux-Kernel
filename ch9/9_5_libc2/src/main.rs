// 현재 시각을 출력하는 예제
use std::mem;
use std::time::{Duration, UNIX_EPOCH};

fn main() {
    // libc::timeval 구조체를 사용하여 현재 시각을 가져오는 예제
    let mut tv = libc::timeval {
        tv_sec: 0, // 초 단위
        tv_usec: 0, // 마이크로초 단위
    };

    // 현재 시스템 시간을 가져온다. unsafe 블록 안에서만 사용 가능
    unsafe {
        libc::gettimeofday(&mut tv, mem::zeroed());
    }

    // tv_sec와 tv_usec를 사용하여 Duration을 생성하고, UNIX_EPOCH에 더하여 SystemTime을 생성
    // tv_sec는 초 단위, tv_usec는 마이크로초 단위이므로, tv_usec를 1000으로 나누어 밀리초 단위로 변환
    let duration = Duration::new(tv.tv_sec as u64, tv.tv_usec as u32 * 1000);
    
    // UNIX_EPOCH는 (1970년 1월 1일 00:00:00 UTC) 로 부터의 경과 시간을 계산
    let system_time = UNIX_EPOCH + duration;

    // libc의 tm 구조체를 초기화 한다.
    let mut tm = unsafe { mem::zeroed() };

    // 시간을 현지 시간대로 변환한다.
    unsafe {
        libc::localtime_r(&system_time as *const _ as *const libc::time_t, &mut tm);
    }

    // 변환된 현지 시간에서 날짜와 시간 구성 요소를 추출한다.
    let day = tm.tm_mday;
    let month = tm.tm_mon + 1; // 0부터 시작
    let year = tm.tm_year + 1900; // 1900부터 시작
    let hour = tm.tm_hour;
    let min = tm.tm_min;
    let sec = tm.tm_sec;

    println!("지금은: {}년 {}월 {}일 {}:{}:{}", year, month, day, hour, min, sec);
}

/*실행결과
지금은: 2025년 5월 5일 23:26:40
*/