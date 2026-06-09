//! # holidays-ru
//!
//! Библиотека для определения рабочих, выходных и праздничных дней в России.
//!
//! ## Основное API
//!
//! Библиотека предоставляет набор pure-функций, которые по дате возвращают
//! [`Resolved<DayFlags>`](Resolved) или [`Resolved<bool>`](Resolved):
//!
//! ```rust,ignore
//! # #[cfg(feature = "chrono")] {
//! use holidays_ru::{flags, is_day_off, Resolved};
//! use chrono::NaiveDate;
//!
//! let date = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();
//! let result = flags(date);
//!
//! match result {
//!     Resolved::Fact(flags) => println!("Официально: {flags:?}"),
//!     Resolved::Predict(flags) => println!("Прогноз: {flags:?}"),
//! }
//! # }
//! ```
//!
//! ## Без внешних зависимостей (ymd API)
//!
//! ```rust
//! use holidays_ru;
//!
//! let result = holidays_ru::flags_ymd(2026, 1, 9).unwrap();
//!
//! if result.value().is_day_off() {
//!     println!("9 января 2026 — выходной день");
//! }
//! ```
//!
//! ## Поддерживаемые годы
//!
//! - **2000–2026**: официальные данные производственного календаря
//!   (возвращаются как [`Resolved::Fact`]).
//! - **1900–9998 вне диапазона официальных данных**: алгоритмический прогноз
//!   на основе ТК РФ (возвращаются как [`Resolved::Predict`]).
//!
//! ## Feature flags
//!
//! - `chrono` — поддержка [`chrono::NaiveDate`]
//! - `time` — поддержка [`time::Date`]
//! - `serde` — сериализация [`DayFlags`] и [`Resolved<T>`]
//!
//! Без фич библиотека работает только через `_ymd` API.

mod data;
mod day_flags;
mod predict;
mod raw_date;
mod resolved;

#[cfg(any(feature = "time", feature = "chrono"))]
pub mod date;

mod official;

pub use day_flags::DayFlags;
pub use resolved::Resolved;

#[cfg(any(feature = "time", feature = "chrono"))]
pub use date::CalendarDate;

/// Первый год, для которого есть официальные данные производственного календаря.
pub const FIRST_FACT_YEAR: i32 = data::FACT_FIRST_YEAR;

/// Последний год, для которого есть официальные данные производственного календаря.
pub const LAST_FACT_YEAR: i32 = data::FACT_LAST_YEAR;

/// Минимальный год, принимаемый `_ymd` API.
pub const MIN_YEAR: i32 = 1900;

/// Максимальный год, принимаемый `_ymd` API.
///
/// Верхняя граница оставляет место для внутреннего просмотра следующего дня
/// в prediction-алгоритме.
pub const MAX_YEAR: i32 = 9998;

use predict as predict_mod;
use raw_date::RawDate;

// ---------------------------------------------------------------------------
// Generic API (CalendarDate)
// ---------------------------------------------------------------------------

#[cfg(any(feature = "time", feature = "chrono"))]
mod generic {
    use super::*;
    use crate::date::CalendarDate;

    /// Возвращает [`DayFlags`] для указанной даты.
    ///
    /// Если для года даты есть официальные данные, возвращается [`Resolved::Fact`].
    /// Иначе — [`Resolved::Predict`] с алгоритмическим прогнозом.
    ///
    /// # Пример
    ///
    /// ```rust,ignore
    /// # #[cfg(feature = "chrono")] {
    /// use holidays_ru::flags;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    /// let result = flags(date);
    ///
    /// assert!(result.is_fact());
    /// assert!(result.value().is_holiday());
    /// # }
    /// ```
    #[inline]
    pub fn flags<D: CalendarDate>(date: D) -> Resolved<DayFlags> {
        let raw = RawDate::from_calendar_date(date);
        super::flags_raw(raw)
    }

    /// Возвращает `true`, если день выходной.
    ///
    /// Объединяет weekend, праздники и дополнительные выходные.
    /// Не различает факт и прогноз — для этого используйте `match` на результате.
    #[inline]
    pub fn is_day_off<D: CalendarDate>(date: D) -> Resolved<bool> {
        flags(date).map(DayFlags::is_day_off)
    }

    /// Возвращает `true`, если день рабочий.
    #[inline]
    pub fn is_working_day<D: CalendarDate>(date: D) -> Resolved<bool> {
        flags(date).map(DayFlags::is_working_day)
    }

    /// Возвращает `true`, если день является федеральным нерабочим праздничным днём.
    #[inline]
    pub fn is_holiday<D: CalendarDate>(date: D) -> Resolved<bool> {
        flags(date).map(DayFlags::is_holiday)
    }

    /// Возвращает `true`, если день является сокращённым рабочим днём.
    #[inline]
    pub fn is_short_day<D: CalendarDate>(date: D) -> Resolved<bool> {
        flags(date).map(DayFlags::is_short_day)
    }

    /// Возвращает `true`, если день — суббота или воскресенье.
    #[inline]
    pub fn is_weekend<D: CalendarDate>(date: D) -> Resolved<bool> {
        flags(date).map(DayFlags::is_weekend)
    }

    /// Возвращает `true`, если день затронут переносом выходного.
    #[inline]
    pub fn is_transferred<D: CalendarDate>(date: D) -> Resolved<bool> {
        flags(date).map(DayFlags::is_transferred)
    }
}

#[cfg(any(feature = "time", feature = "chrono"))]
pub use generic::{
    flags, is_day_off, is_holiday, is_short_day, is_transferred, is_weekend, is_working_day,
};

// ---------------------------------------------------------------------------
// YMD API (без внешних зависимостей)
// ---------------------------------------------------------------------------

/// Возвращает [`DayFlags`] для даты, заданной годом, месяцем и днём.
///
/// Возвращает `None`, если дата недействительна
/// (например, 31 февраля, 29 февраля в невисокосный год или год вне
/// [`MIN_YEAR`]..=[`MAX_YEAR`]).
///
/// # Пример
///
/// ```rust
/// use holidays_ru;
///
/// let result = holidays_ru::flags_ymd(2026, 1, 9).unwrap();
/// assert!(result.value().is_day_off());
/// ```
#[inline]
pub fn flags_ymd(year: i32, month: u8, day: u8) -> Option<Resolved<DayFlags>> {
    let raw = RawDate::from_ymd(year, month, day)?;
    Some(flags_raw(raw))
}

/// Возвращает `true`, если день выходной.
///
/// `None` означает невалидную дату.
#[inline]
pub fn is_day_off_ymd(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd(year, month, day).map(|r| r.map(DayFlags::is_day_off))
}

/// Возвращает `true`, если день рабочий.
///
/// `None` означает невалидную дату.
#[inline]
pub fn is_working_day_ymd(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd(year, month, day).map(|r| r.map(DayFlags::is_working_day))
}

/// Возвращает `true`, если день является федеральным нерабочим праздничным днём.
///
/// `None` означает невалидную дату.
#[inline]
pub fn is_holiday_ymd(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd(year, month, day).map(|r| r.map(DayFlags::is_holiday))
}

/// Возвращает `true`, если день является сокращённым рабочим днём.
///
/// `None` означает невалидную дату.
#[inline]
pub fn is_short_day_ymd(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd(year, month, day).map(|r| r.map(DayFlags::is_short_day))
}

/// Возвращает `true`, если день — суббота или воскресенье.
///
/// `None` означает невалидную дату.
#[inline]
pub fn is_weekend_ymd(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd(year, month, day).map(|r| r.map(DayFlags::is_weekend))
}

/// Возвращает `true`, если день затронут переносом выходного.
///
/// `None` означает невалидную дату.
#[inline]
pub fn is_transferred_ymd(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd(year, month, day).map(|r| r.map(DayFlags::is_transferred))
}

// ---------------------------------------------------------------------------
// Внутренняя логика разрешения (Fact vs Predict)
// ---------------------------------------------------------------------------

/// Основной поток разрешения: пытается получить фактические данные,
/// иначе использует прогноз.
#[inline]
pub(crate) fn flags_raw(date: RawDate) -> Resolved<DayFlags> {
    if let Some(flags) = official::flags(date) {
        Resolved::Fact(flags)
    } else {
        Resolved::Predict(predict_mod::flags(date))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // YMD API tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_flags_ymd_valid() {
        let r = flags_ymd(2026, 1, 9).unwrap();
        assert!(r.is_fact());
        assert!(r.value().is_day_off());
    }

    #[test]
    fn test_flags_ymd_invalid() {
        assert!(flags_ymd(2026, 2, 31).is_none());
        assert!(flags_ymd(2026, 13, 1).is_none());
        assert!(flags_ymd(2026, 1, 0).is_none());
    }

    #[test]
    fn test_flags_ymd_predict() {
        // 2027 год — за пределами официальных данных.
        let r = flags_ymd(2027, 1, 1).unwrap();
        assert!(r.is_predict());
        assert!(r.value().is_holiday());
    }

    #[test]
    fn test_is_day_off_ymd() {
        let r = is_day_off_ymd(2026, 1, 9).unwrap();
        assert!(r.value());
    }

    #[test]
    fn test_is_holiday_ymd() {
        let r = is_holiday_ymd(2026, 1, 1).unwrap();
        assert!(r.value());

        let r = is_holiday_ymd(2026, 1, 9).unwrap();
        assert!(!r.value());
    }

    #[test]
    fn test_is_working_day_ymd() {
        let r = is_working_day_ymd(2026, 1, 12).unwrap();
        // 12 января 2026 — понедельник, после каникул, рабочий день.
        assert!(r.value());
    }

    #[test]
    fn test_is_short_day_ymd() {
        // 3 ноября 2026 — вторник, короткий день перед 4 ноября.
        let r = is_short_day_ymd(2026, 11, 3).unwrap();
        assert!(r.value());
    }

    #[test]
    fn test_is_weekend_ymd() {
        // 11 января 2026 — воскресенье.
        let r = is_weekend_ymd(2026, 1, 11).unwrap();
        assert!(r.value());
    }

    #[test]
    fn test_flags_ymd_year_range() {
        assert!(flags_ymd(MIN_YEAR - 1, 1, 1).is_none());
        assert!(flags_ymd(MIN_YEAR, 1, 1).is_some());
        assert!(flags_ymd(MAX_YEAR, 12, 31).is_some());
        assert!(flags_ymd(MAX_YEAR + 1, 1, 1).is_none());
    }

    #[test]
    fn test_fact_year_invariants() {
        for year in FIRST_FACT_YEAR..=LAST_FACT_YEAR {
            let mut date = RawDate::from_ymd(year, 1, 1).unwrap();

            loop {
                let flags = flags_raw(date).value();

                assert_ne!(
                    flags.is_day_off(),
                    flags.is_working_day(),
                    "{year}-{:02}-{:02}: day cannot be both off and working",
                    date.month,
                    date.day,
                );
                assert!(
                    !flags.is_short_day() || flags.is_working_day(),
                    "{year}-{:02}-{:02}: short day must be working",
                    date.month,
                    date.day,
                );
                assert!(
                    !flags.is_holiday() || flags.is_day_off(),
                    "{year}-{:02}-{:02}: holiday must be day off",
                    date.month,
                    date.day,
                );

                if date.month == 12 && date.day == 31 {
                    break;
                }

                date = date.next_day();
            }
        }
    }

    // -----------------------------------------------------------------------
    // chrono feature tests
    // -----------------------------------------------------------------------

    #[cfg(feature = "chrono")]
    mod chrono_tests {
        use super::*;
        use chrono::NaiveDate;

        #[test]
        fn test_flags_with_naive_date() {
            let date = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();
            let r = flags(date);
            assert!(r.is_fact());
            assert!(r.value().is_day_off());
        }

        #[test]
        fn test_is_day_off_with_naive_date() {
            let date = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();
            assert!(is_day_off(date).value());
        }

        #[test]
        fn test_predict_with_naive_date() {
            let date = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
            let r = flags(date);
            assert!(r.is_predict());
            assert!(r.value().is_holiday());
        }

        #[test]
        fn test_chrono_matches_ymd() {
            for (year, month, day) in [
                (2000, 1, 1),
                (2010, 1, 6),
                (2024, 4, 27),
                (2026, 12, 31),
                (2027, 1, 11),
            ] {
                let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
                assert_eq!(
                    flags(date),
                    flags_ymd(year, month as u8, day as u8).unwrap()
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // time feature tests
    // -----------------------------------------------------------------------

    #[cfg(feature = "time")]
    mod time_tests {
        use super::*;
        use time::Date;
        use time::Month;

        #[test]
        fn test_flags_with_time_date() {
            let date = Date::from_calendar_date(2026, Month::January, 9).unwrap();
            let r = flags(date);
            assert!(r.is_fact());
            assert!(r.value().is_day_off());
        }

        #[test]
        fn test_is_day_off_with_time_date() {
            let date = Date::from_calendar_date(2026, Month::January, 9).unwrap();
            assert!(is_day_off(date).value());
        }

        #[test]
        fn test_predict_with_time_date() {
            let date = Date::from_calendar_date(2027, Month::January, 1).unwrap();
            let r = flags(date);
            assert!(r.is_predict());
            assert!(r.value().is_holiday());
        }

        #[test]
        fn test_time_matches_ymd() {
            for (year, month, day) in [
                (2000, Month::January, 1),
                (2010, Month::January, 6),
                (2024, Month::April, 27),
                (2026, Month::December, 31),
                (2027, Month::January, 11),
            ] {
                let date = Date::from_calendar_date(year, month, day).unwrap();
                assert_eq!(flags(date), flags_ymd(year, month.into(), day).unwrap());
            }
        }
    }
}
