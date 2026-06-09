/// Трейт для типов дат из внешних крейтов.
///
/// Этот трейт запечатан (`sealed`) — пользователь не может реализовать его
/// для своих типов. Библиотека предоставляет реализации для:
///
/// - `time::Date` (за feature `time`)
/// - `chrono::NaiveDate` (за feature `chrono`)
///
/// Если ни одна из этих фич не включена, используйте API на основе `_ymd`:
///
/// ```rust
/// use holidays_ru;
///
/// let result = holidays_ru::flags_ymd(2026, 1, 9).unwrap();
/// ```
pub trait CalendarDate: sealed::Sealed + Copy {
    /// Год.
    fn year(self) -> i32;

    /// Месяц (1–12).
    fn month(self) -> u8;

    /// День месяца (1–31).
    fn day(self) -> u8;

    /// Порядковый номер дня в году, начиная с 0 (1 января = 0).
    fn ordinal0(self) -> u16;

    /// День недели: 0 = понедельник, ..., 6 = воскресенье.
    fn weekday_number_from_monday(self) -> u8;
}

/// Модуль для запечатывания трейта `CalendarDate`.
mod sealed {
    /// Супертрейт, который невозможно реализовать вне крейта.
    pub trait Sealed {}

    #[cfg(feature = "time")]
    impl Sealed for time::Date {}

    #[cfg(feature = "chrono")]
    impl Sealed for chrono::NaiveDate {}
}

#[cfg(feature = "chrono")]
use chrono::{Datelike, Weekday};

#[cfg(feature = "chrono")]
impl CalendarDate for chrono::NaiveDate {
    #[inline]
    fn year(self) -> i32 {
        Datelike::year(&self)
    }

    #[inline]
    fn month(self) -> u8 {
        Datelike::month(&self) as u8
    }

    #[inline]
    fn day(self) -> u8 {
        Datelike::day(&self) as u8
    }

    #[inline]
    fn ordinal0(self) -> u16 {
        (Datelike::ordinal0(&self)) as u16
    }

    #[inline]
    fn weekday_number_from_monday(self) -> u8 {
        match self.weekday() {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
        }
    }
}

#[cfg(feature = "time")]
impl CalendarDate for time::Date {
    #[inline]
    fn year(self) -> i32 {
        time::Date::year(self)
    }

    #[inline]
    fn month(self) -> u8 {
        time::Date::month(self).into()
    }

    #[inline]
    fn day(self) -> u8 {
        time::Date::day(self)
    }

    #[inline]
    fn ordinal0(self) -> u16 {
        time::Date::ordinal(self) - 1
    }

    #[inline]
    fn weekday_number_from_monday(self) -> u8 {
        use time::Weekday;
        match self.weekday() {
            Weekday::Monday => 0,
            Weekday::Tuesday => 1,
            Weekday::Wednesday => 2,
            Weekday::Thursday => 3,
            Weekday::Friday => 4,
            Weekday::Saturday => 5,
            Weekday::Sunday => 6,
        }
    }
}
