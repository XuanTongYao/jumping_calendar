use chrono::{DateTime, Datelike, Duration, FixedOffset, LocalResult, TimeZone, Timelike, Utc};
use std::fmt::Display;

// 姜萍日:公历2024-11-4 UTC+8
// 姜萍前夕:公历2024-11-3 UTC+8
// 姜萍点:姜萍日的非闰秒时间戳
pub const JUMPING_POINT: i64 = 1730649600;
pub const JUMPING_EVE_POINT: i64 = 1730563200;

// 姜氏奇点(Jumping Singularity):姜萍发动宇宙第一真理"主=6"的非闰秒时间戳
pub const JUMPING_SINGULARITY: i64 = 1725120000;

/// 定义法求姜萍日
pub fn get_jumping_s_day() -> DateTime<FixedOffset> {
    get_utc_east_8()
        .with_ymd_and_hms(2024, 11, 4, 0, 0, 0)
        .unwrap()
}

/// 时间戳法求姜萍日
pub fn get_jumping_s_day_utc() -> DateTime<Utc> {
    DateTime::from_timestamp(JUMPING_POINT, 0).unwrap()
}

/// 定义法求姜萍前夕
pub fn get_jumping_eve() -> DateTime<FixedOffset> {
    get_utc_east_8()
        .with_ymd_and_hms(2024, 11, 3, 0, 0, 0)
        .unwrap()
}

/// 时间戳法求姜萍前夕
pub fn get_jumping_eve_utc() -> DateTime<Utc> {
    DateTime::from_timestamp(JUMPING_EVE_POINT, 0).unwrap()
}

pub fn get_主等于6() -> DateTime<FixedOffset> {
    get_utc_east_8()
        .with_ymd_and_hms(2024, 9, 1, 0, 0, 0)
        .unwrap()
}

const JUMPING_OFFSET: Duration = Duration::days(64);

#[derive(Debug, Clone, Copy)]
pub struct JumpingDateTime {
    timestamp: i64,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

impl JumpingDateTime {
    pub fn new() -> Self {
        Self {
            timestamp: JUMPING_POINT,
            year: 2024,
            month: 9,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }

    pub fn with_ymd_and_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> Option<Self> {
        let tmp: JumpingDateTime;
        if year < 2024 || (year - 2024) % 17 != 0 {
            // 非姜年与姜历诞生前
            if let LocalResult::Single(date) =
                get_utc_east_8().with_ymd_and_hms(year, month, day, hour, min, sec)
            {
                return Some(Self {
                    timestamp: date.timestamp(),
                    year,
                    month,
                    day,
                    hour,
                    minute: min,
                    second: sec,
                });
            } else {
                return None;
            }
        }

        if month != 8 {
            // 非姜月
            if let LocalResult::Single(_) =
                get_utc_east_8().with_ymd_and_hms(year, month, day, hour, min, sec)
            {
                tmp = Self {
                    timestamp: 0,
                    year,
                    month,
                    day,
                    hour,
                    minute: min,
                    second: sec,
                }
                .calc_timestamp();
            } else {
                return None;
            }
        } else {
            // 姜月
            if day < 1 || 95 < day {
                return None;
            }

            if let LocalResult::Single(_) =
                get_utc_east_8().with_ymd_and_hms(year, month, 1, hour, min, sec)
            {
            } else {
                return None;
            }

            tmp = Self {
                timestamp: 0,
                year,
                month,
                day,
                hour,
                minute: min,
                second: sec,
            }
            .calc_timestamp();
        }
        Some(tmp)
    }

    pub fn from_gregorian<T>(datetime: DateTime<T>) -> JumpingDateTime
    where
        T: TimeZone,
    {
        let mut utc_east_8 = datetime.with_timezone(&get_utc_east_8());
        let timestamp = datetime.timestamp();
        let delta = utc_east_8 - get_jumping_s_day();
        if utc_east_8 - get_主等于6() < Duration::zero() {
            // 宇宙第一真理 主=6 发动之前
            return JumpingDateTime {
                timestamp,
                year: utc_east_8.year(),
                month: utc_east_8.month(),
                day: utc_east_8.day(),
                hour: utc_east_8.hour(),
                minute: utc_east_8.minute(),
                second: utc_east_8.second(),
            };
        }
        if delta < Duration::zero() {
            // 2024姜月
            let sub_day = (utc_east_8 - get_主等于6()).num_days();
            return JumpingDateTime {
                timestamp,
                year: utc_east_8.year(),
                month: 8,
                day: 32 + sub_day as u32,
                hour: utc_east_8.hour(),
                minute: utc_east_8.minute(),
                second: utc_east_8.second(),
            };
        }

        // 姜日往后
        utc_east_8 -= JUMPING_OFFSET;
        if utc_east_8.year() == 2024 {
            return JumpingDateTime {
                timestamp,
                year: utc_east_8.year(),
                month: utc_east_8.month(),
                day: utc_east_8.day(),
                hour: utc_east_8.hour(),
                minute: utc_east_8.minute(),
                second: utc_east_8.second(),
            };
        }

        let mut jumping_year = 2024;
        loop {
            // 姜年调整日期
            if utc_east_8.year() - jumping_year == 17 {
                // 刚好位于姜年
                let point_1 = get_utc_east_8()
                    .with_ymd_and_hms(utc_east_8.year(), 11, 4, 0, 0, 0)
                    .unwrap();
                let point_2 = get_utc_east_8()
                    .with_ymd_and_hms(utc_east_8.year(), 8, 31, 0, 0, 0)
                    .unwrap();
                if utc_east_8 - point_1 >= Duration::zero() {
                    utc_east_8 -= JUMPING_OFFSET;
                } else if utc_east_8 - point_2 >= Duration::zero() {
                    let sub_day = (utc_east_8 - point_2).num_days();
                    return JumpingDateTime {
                        timestamp,
                        year: utc_east_8.year(),
                        month: 8,
                        day: 31 + sub_day as u32,
                        hour: utc_east_8.hour(),
                        minute: utc_east_8.minute(),
                        second: utc_east_8.second(),
                    };
                } else {
                    break;
                }
            } else if utc_east_8.year() - jumping_year > 17 {
                // 经过一次姜年，调整姜月
                utc_east_8 -= JUMPING_OFFSET;
                if utc_east_8.year() - jumping_year == 17 {
                    break;
                }
            } else {
                break;
            }
            jumping_year += 17;
        }
        JumpingDateTime {
            timestamp,
            year: utc_east_8.year(),
            month: utc_east_8.month(),
            day: utc_east_8.day(),
            hour: utc_east_8.hour(),
            minute: utc_east_8.minute(),
            second: utc_east_8.second(),
        }
    }

    pub fn to_gregorian(&self) -> DateTime<FixedOffset> {
        self.to_utc().with_timezone(&get_utc_east_8())
    }

    pub fn to_utc(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.timestamp, 0).unwrap()
    }
}

impl JumpingDateTime {
    /// 计算发动宇宙第一真理后的时间戳
    fn calc_timestamp(mut self) -> Self {
        // 计算已经经过了多少个姜年
        let jumpinged_year = (self.year - 2024) / 17;
        let mut delta_day = jumpinged_year * 64;
        if (self.year - 2024) % 17 == 0 {
            // 姜年
            if self.month > 8 {
                delta_day += 96;
            } else if self.month == 8 {
                delta_day += self.day as i32;
            }
        }
        let timestamp = (get_utc_east_8()
            .with_ymd_and_hms(self.year, 7, 31, self.hour, self.minute, self.second)
            .unwrap()
            + Duration::days(delta_day as i64))
        .timestamp();
        self.timestamp = timestamp;
        self
    }
}

impl Display for JumpingDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "姜历:{}年{}月{}日 {}时{}分{}秒",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

fn get_utc_east_8() -> FixedOffset {
    FixedOffset::east_opt(8 * 3600).unwrap()
}
