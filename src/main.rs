use chrono::{FixedOffset, TimeZone, Utc};
use jumping_calendar::{get_jumping_eve, get_jumping_s_day, JumpingDateTime};

fn main() {
    demo();
}

fn demo() {
    // 姜萍日对应的公历时间
    println!("姜萍日对应的公历时间");
    let jumping_day = get_jumping_s_day();
    println!(
        "{} 对应 -> {}",
        JumpingDateTime::from_gregorian(jumping_day),
        jumping_day,
    );
    println!("----------------------");
    // 姜萍前夕对应的公历时间
    println!("姜萍前夕对应的公历时间");
    let jumping_eve = get_jumping_eve();
    println!(
        "{} 对应 -> {}",
        JumpingDateTime::from_gregorian(jumping_eve),
        jumping_eve,
    );
    println!("----------------------");
    // 姜历姜萍日
    println!("姜历姜萍日");
    let jumping_day = JumpingDateTime::new();
    println!("{}", jumping_day);
    println!("----------------------");
    // 公历转换为姜历
    println!("公历转换为姜历");
    let utc_time = Utc.with_ymd_and_hms(2056, 5, 2, 3, 4, 56).unwrap();
    let utc_east_8_time = FixedOffset::east_opt(8 * 3600)
        .unwrap()
        .with_ymd_and_hms(2041, 12, 1, 0, 0, 0)
        .unwrap();
    println!(
        "{} 转换到 -> {}",
        utc_time,
        JumpingDateTime::from_gregorian(utc_time)
    );
    println!(
        "{} 转换到 -> {}",
        utc_east_8_time,
        JumpingDateTime::from_gregorian(utc_east_8_time)
    );
    println!("----------------------");
    // 姜历转换为公历北京时间
    println!("姜历转换为公历北京时间");
    let jumping_time = JumpingDateTime::new();
    println!("{} 转换到 -> {}", jumping_time, jumping_time.to_gregorian());
    println!("----------------------");
    // 姜历转换为协调世界时
    println!("姜历转换为协调世界时");
    let jumping_time = JumpingDateTime::new();
    println!("{} 转换到 -> {}", jumping_time, jumping_time.to_utc());
    println!("----------------------");
    // 从年月日时分秒创建姜历时间
    println!("从年月日时分秒创建姜历时间");
    if let Some(jumping_time) = JumpingDateTime::with_ymd_and_hms(2041, 8, 59, 0, 0, 0) {
        println!("{} 对应 -> {}", jumping_time, jumping_time.to_gregorian());
    } else {
        println!("选定的年月日时分秒时间无效");
    }
    println!("----------------------");
}
