use std::time::{SystemTime, Duration};

fn main() {
    let now = SystemTime::now();
    let after = now + Duration::from_secs(3);

    println!("현재시간: {:?}", now);
    println!("+3초: {:?}", after);
}

/*실행결과
현재시간: SystemTime { intervals: 133901387700869013 }
    +3초: SystemTime { intervals: 133901387730869013 }
*/