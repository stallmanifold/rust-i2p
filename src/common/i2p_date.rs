use common::i2p_integer::I2pInt64;
use chrono::datetime::DateTime;
use chrono::naive::datetime::NaiveDateTime;
use chrono::offset::utc;
use std::fmt;
use std::u64;
use rand;


/// The `Date` type counts the number of milliseconds since January 1, 1970 (UNIX time)
/// in the GMT timezone. If the number is 0, the date is undefined or null.
///
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Debug)]
pub struct I2pDate {
    milliseconds: I2pInt64
}

impl I2pDate {
    pub fn new(milliseconds: I2pInt64) -> I2pDate {
        if milliseconds == I2pInt64::from(0) {
            panic!("Got a zero value for milliseconds.");
        }

        I2pDate {
            milliseconds: milliseconds
        }
    }

    pub fn to_rfc3339(&self) -> String {
        let datetime: DateTime<utc::UTC> = self.to_datetime();

        datetime.to_rfc3339()
    }

    pub fn to_datetime(&self) -> DateTime<utc::UTC> {
        let msec_to_sec  = I2pInt64::new(1000);
        let msec_to_nsec = I2pInt64::new(1_000_000);
        let seconds      = self.milliseconds / msec_to_sec;
        let remainder    = self.milliseconds % msec_to_sec;
        let nanoseconds  = remainder * msec_to_nsec;
        let naive_dt     = NaiveDateTime::from_timestamp(seconds.to_u64() as i64, nanoseconds.to_u64() as u32);

        DateTime::from_utc(naive_dt, utc::UTC)
    }

    pub fn min_value() -> I2pDate {
        I2pDate::new(I2pInt64::new(1))
    }

    pub fn max_value() -> I2pDate {
        I2pDate::new(I2pInt64::new(u64::max_value()))
    }
}

impl Default for I2pDate {
    fn default() -> I2pDate {
        I2pDate::min_value()
    }
}

impl fmt::Display for I2pDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_rfc3339())
    }
}

impl rand::Rand for I2pDate {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        let random_date = I2pInt64::new(rng.next_u64());

        I2pDate::new(random_date)
    }
}

#[cfg(test)]
mod tests {
    use super::I2pDate;
    use common::i2p_integer::I2pInt64;


    #[test]
    #[should_panic]
    fn test_constructor_should_panic_when_milliseconds_is_zero() {
        let date = I2pDate::new(I2pInt64::new(0));

        // This test fails if no panic occurs.
        assert!(false);
    }
}
