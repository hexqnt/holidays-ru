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
//! use holidays_ru::{Federal, Resolved, flags, is_day_off};
//! use chrono::NaiveDate;
//!
//! let date = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();
//! let result = flags::<Federal, _>(date);
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
//! let result = holidays_ru::flags_ymd::<holidays_ru::Federal>(2026, 1, 9).unwrap();
//!
//! if result.value().is_day_off() {
//!     println!("9 января 2026 — выходной день");
//! }
//! ```
//!
//! ## Поддерживаемые годы
//!
//! - **1993–2026**: официальные данные производственного календаря
//!   (возвращаются как [`Resolved::Fact`]).
//! - **1900–2100 вне диапазона официальных данных**: алгоритмический прогноз
//!   на основе ТК РФ (возвращаются как [`Resolved::Predict`]).
//!
//! ## Feature flags
//!
//! - `chrono` — поддержка [`chrono::NaiveDate`]
//! - `time` — поддержка [`time::Date`]
//! - `serde` — сериализация [`DayFlags`] и [`Resolved<T>`]
//!
//! Без фич библиотека работает только через `_ymd` API.

use predict as predict_mod;
use raw_date::RawDate;

pub use calendar::regions;
pub use calendar::{Calendar, Federal};
pub use day_flags::DayFlags;
pub use range::WorkWeek;
pub use resolved::Resolved;

#[cfg(any(feature = "time", feature = "chrono"))]
pub use date::CalendarDate;

mod calendar;
mod data;
mod day_flags;
mod predict;
mod range;
mod raw_date;
mod resolved;

#[cfg(any(feature = "time", feature = "chrono"))]
pub mod date;

mod official;

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
pub const MAX_YEAR: i32 = 2100;

// ---------------------------------------------------------------------------
// Generic API (CalendarDate)
// ---------------------------------------------------------------------------

#[cfg(any(feature = "time", feature = "chrono"))]
pub use generic::{
    flags, is_day_off, is_holiday, is_short_day, is_transferred, is_weekend, is_working_day,
    non_working_days_between, working_hours_between, working_minutes_between,
};

#[cfg(any(feature = "time", feature = "chrono"))]
mod generic {
    use crate::date::CalendarDate;

    use super::{Calendar, DayFlags, RawDate, Resolved, WorkWeek, range};

    /// Возвращает [`DayFlags`] для указанной даты.
    ///
    /// Если для года даты есть официальные данные, возвращается [`Resolved::Fact`].
    /// Иначе — [`Resolved::Predict`] с алгоритмическим прогнозом.
    ///
    /// # Пример
    ///
    /// ```rust,ignore
    /// # #[cfg(feature = "chrono")] {
    /// use holidays_ru::{Federal, flags};
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    /// let result = flags::<Federal, _>(date).unwrap();
    ///
    /// assert!(result.is_fact());
    /// assert!(result.value().is_holiday());
    /// # }
    /// ```
    #[inline]
    #[must_use]
    pub fn flags<C: Calendar, D: CalendarDate>(date: D) -> Option<Resolved<DayFlags>> {
        let raw = RawDate::from_calendar_date(date);
        C::flags_ymd(raw.year, raw.month, raw.day)
    }

    /// Возвращает `true`, если день выходной.
    ///
    /// Объединяет weekend, праздники и дополнительные выходные.
    /// Не различает факт и прогноз — для этого используйте `match` на результате.
    #[inline]
    #[must_use]
    pub fn is_day_off<C: Calendar, D: CalendarDate>(date: D) -> Option<Resolved<bool>> {
        flags::<C, _>(date).map(|r| r.map(DayFlags::is_day_off))
    }

    /// Возвращает `true`, если день рабочий.
    #[inline]
    #[must_use]
    pub fn is_working_day<C: Calendar, D: CalendarDate>(date: D) -> Option<Resolved<bool>> {
        flags::<C, _>(date).map(|r| r.map(DayFlags::is_working_day))
    }

    /// Возвращает `true`, если день является федеральным нерабочим праздничным днём.
    #[inline]
    #[must_use]
    pub fn is_holiday<C: Calendar, D: CalendarDate>(date: D) -> Option<Resolved<bool>> {
        flags::<C, _>(date).map(|r| r.map(DayFlags::is_holiday))
    }

    /// Возвращает `true`, если день является сокращённым рабочим днём.
    #[inline]
    #[must_use]
    pub fn is_short_day<C: Calendar, D: CalendarDate>(date: D) -> Option<Resolved<bool>> {
        flags::<C, _>(date).map(|r| r.map(DayFlags::is_short_day))
    }

    /// Возвращает `true`, если день — суббота или воскресенье.
    #[inline]
    #[must_use]
    pub fn is_weekend<C: Calendar, D: CalendarDate>(date: D) -> Option<Resolved<bool>> {
        flags::<C, _>(date).map(|r| r.map(DayFlags::is_weekend))
    }

    /// Возвращает `true`, если день затронут переносом выходного.
    #[inline]
    #[must_use]
    pub fn is_transferred<C: Calendar, D: CalendarDate>(date: D) -> Option<Resolved<bool>> {
        flags::<C, _>(date).map(|r| r.map(DayFlags::is_transferred))
    }

    /// Считает нерабочие дни в полуоткрытом диапазоне дат `[start, end)`.
    ///
    /// Возвращает `None`, если `start > end` или даты вне поддерживаемого
    /// диапазона. Для `end` дополнительно допускается `MAX_YEAR + 1`-01-01,
    /// чтобы диапазон мог включать [`crate::MAX_YEAR`]-12-31.
    /// Если хотя бы один день диапазона рассчитан прогнозом, итоговый результат
    /// будет [`Resolved::Predict`].
    #[inline]
    #[must_use]
    pub fn non_working_days_between<C: Calendar, D: CalendarDate>(
        start: D,
        end: D,
    ) -> Option<Resolved<u32>> {
        let start = RawDate::from_calendar_date(start);
        let end = RawDate::from_calendar_date(end);

        range::non_working_days_between_raw::<C>(start, end)
    }

    /// Считает рабочее время в минутах в полуоткрытом диапазоне дат `[start, end)`.
    ///
    /// Нерабочие дни дают 0 минут. Сокращённые рабочие дни уменьшают норму
    /// выбранной рабочей недели на 60 минут. Для региональных overlay-календарей
    /// возвращается `None`, поскольку они не задают полный рабочий график.
    #[inline]
    #[must_use]
    pub fn working_minutes_between<C: Calendar, D: CalendarDate>(
        start: D,
        end: D,
        week: WorkWeek,
    ) -> Option<Resolved<u32>> {
        let start = RawDate::from_calendar_date(start);
        let end = RawDate::from_calendar_date(end);

        range::working_minutes_between_raw::<C>(start, end, week)
    }

    /// Считает рабочее время в часах в полуоткрытом диапазоне дат `[start, end)`.
    ///
    /// Для точных расчётов используйте [`working_minutes_between`]. Для
    /// региональных overlay-календарей возвращается `None`.
    #[inline]
    #[must_use]
    pub fn working_hours_between<C: Calendar, D: CalendarDate>(
        start: D,
        end: D,
        week: WorkWeek,
    ) -> Option<Resolved<f64>> {
        working_minutes_between::<C, _>(start, end, week)
            .map(|r| r.map(|minutes| f64::from(minutes) / 60.0))
    }
}

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
/// let result = holidays_ru::flags_ymd::<holidays_ru::Federal>(2026, 1, 9).unwrap();
/// assert!(result.value().is_day_off());
/// ```
#[inline]
#[must_use]
pub fn flags_ymd<C: Calendar>(year: i32, month: u8, day: u8) -> Option<Resolved<DayFlags>> {
    C::flags_ymd(year, month, day)
}

/// Возвращает `true`, если день выходной.
///
/// `None` означает невалидную дату.
#[inline]
#[must_use]
pub fn is_day_off_ymd<C: Calendar>(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd::<C>(year, month, day).map(|r| r.map(DayFlags::is_day_off))
}

/// Возвращает `true`, если день рабочий.
///
/// `None` означает невалидную дату.
#[inline]
#[must_use]
pub fn is_working_day_ymd<C: Calendar>(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd::<C>(year, month, day).map(|r| r.map(DayFlags::is_working_day))
}

/// Возвращает `true`, если день является федеральным нерабочим праздничным днём.
///
/// `None` означает невалидную дату.
#[inline]
#[must_use]
pub fn is_holiday_ymd<C: Calendar>(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd::<C>(year, month, day).map(|r| r.map(DayFlags::is_holiday))
}

/// Возвращает `true`, если день является сокращённым рабочим днём.
///
/// `None` означает невалидную дату.
#[inline]
#[must_use]
pub fn is_short_day_ymd<C: Calendar>(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd::<C>(year, month, day).map(|r| r.map(DayFlags::is_short_day))
}

/// Возвращает `true`, если день — суббота или воскресенье.
///
/// `None` означает невалидную дату.
#[inline]
#[must_use]
pub fn is_weekend_ymd<C: Calendar>(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd::<C>(year, month, day).map(|r| r.map(DayFlags::is_weekend))
}

/// Возвращает `true`, если день затронут переносом выходного.
///
/// `None` означает невалидную дату.
#[inline]
#[must_use]
pub fn is_transferred_ymd<C: Calendar>(year: i32, month: u8, day: u8) -> Option<Resolved<bool>> {
    flags_ymd::<C>(year, month, day).map(|r| r.map(DayFlags::is_transferred))
}

/// Считает нерабочие дни в полуоткрытом диапазоне дат `[start, end)`.
///
/// Возвращает `None`, если одна из дат недействительна или `start > end`.
/// Для `end` дополнительно допускается `MAX_YEAR + 1`-01-01, чтобы диапазон
/// мог включать [`MAX_YEAR`]-12-31.
/// Если хотя бы один день диапазона рассчитан прогнозом, итоговый результат
/// будет [`Resolved::Predict`].
#[inline]
#[must_use]
pub fn non_working_days_between_ymd<C: Calendar>(
    start_year: i32,
    start_month: u8,
    start_day: u8,
    end_year: i32,
    end_month: u8,
    end_day: u8,
) -> Option<Resolved<u32>> {
    let start = RawDate::from_ymd(start_year, start_month, start_day)?;
    let end = range_end_raw_ymd(end_year, end_month, end_day)?;

    range::non_working_days_between_raw::<C>(start, end)
}

/// Считает рабочее время в минутах в полуоткрытом диапазоне дат `[start, end)`.
///
/// Нерабочие дни дают 0 минут. Сокращённые рабочие дни уменьшают норму
/// выбранной рабочей недели на 60 минут. Возвращает `None`, если одна из дат
/// недействительна или `start > end`. Для `end` дополнительно допускается
/// `MAX_YEAR + 1`-01-01, чтобы диапазон мог включать [`MAX_YEAR`]-12-31.
///
/// Расчёт доступен только для полных календарей, таких как [`Federal`]. Для
/// региональных overlay-календарей возвращается `None`, поскольку они содержат
/// только региональные исключения и не задают полный рабочий график.
#[inline]
#[must_use]
pub fn working_minutes_between_ymd<C: Calendar>(
    start_year: i32,
    start_month: u8,
    start_day: u8,
    end_year: i32,
    end_month: u8,
    end_day: u8,
    week: WorkWeek,
) -> Option<Resolved<u32>> {
    let start = RawDate::from_ymd(start_year, start_month, start_day)?;
    let end = range_end_raw_ymd(end_year, end_month, end_day)?;

    range::working_minutes_between_raw::<C>(start, end, week)
}

/// Считает рабочее время в часах в полуоткрытом диапазоне дат `[start, end)`.
///
/// Для точных расчётов используйте [`working_minutes_between_ymd`]. Для
/// региональных overlay-календарей возвращается `None`.
#[inline]
#[must_use]
pub fn working_hours_between_ymd<C: Calendar>(
    start_year: i32,
    start_month: u8,
    start_day: u8,
    end_year: i32,
    end_month: u8,
    end_day: u8,
    week: WorkWeek,
) -> Option<Resolved<f64>> {
    working_minutes_between_ymd::<C>(
        start_year,
        start_month,
        start_day,
        end_year,
        end_month,
        end_day,
        week,
    )
    .map(|r| r.map(|minutes| f64::from(minutes) / 60.0))
}

#[inline]
fn range_end_raw_ymd(year: i32, month: u8, day: u8) -> Option<RawDate> {
    RawDate::from_ymd(year, month, day).or_else(|| {
        (year == MAX_YEAR + 1 && month == 1 && day == 1)
            .then(|| RawDate::from_ymd_unchecked(year, month, day))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // YMD API tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_flags_ymd_valid() {
        let r = flags_ymd::<Federal>(2026, 1, 9).unwrap();
        assert!(r.is_fact());
        assert!(r.value().is_day_off());
    }

    #[test]
    fn test_flags_ymd_invalid() {
        assert!(flags_ymd::<Federal>(2026, 2, 31).is_none());
        assert!(flags_ymd::<Federal>(2026, 13, 1).is_none());
        assert!(flags_ymd::<Federal>(2026, 1, 0).is_none());
    }

    #[test]
    fn test_flags_ymd_predict() {
        // 2027 год — за пределами официальных данных.
        let r = flags_ymd::<Federal>(2027, 1, 1).unwrap();
        assert!(r.is_predict());
        assert!(r.value().is_holiday());
    }

    #[test]
    fn test_is_day_off_ymd() {
        let r = is_day_off_ymd::<Federal>(2026, 1, 9).unwrap();
        assert!(r.value());
    }

    #[test]
    fn test_is_holiday_ymd() {
        let r = is_holiday_ymd::<Federal>(2026, 1, 1).unwrap();
        assert!(r.value());

        let r = is_holiday_ymd::<Federal>(2026, 1, 9).unwrap();
        assert!(!r.value());
    }

    #[test]
    fn test_is_working_day_ymd() {
        let r = is_working_day_ymd::<Federal>(2026, 1, 12).unwrap();
        // 12 января 2026 — понедельник, после каникул, рабочий день.
        assert!(r.value());
    }

    #[test]
    fn test_is_short_day_ymd() {
        // 3 ноября 2026 — вторник, короткий день перед 4 ноября.
        let r = is_short_day_ymd::<Federal>(2026, 11, 3).unwrap();
        assert!(r.value());
    }

    #[test]
    fn test_is_weekend_ymd() {
        // 11 января 2026 — воскресенье.
        let r = is_weekend_ymd::<Federal>(2026, 1, 11).unwrap();
        assert!(r.value());
    }

    #[test]
    fn test_flags_ymd_year_range() {
        assert!(flags_ymd::<Federal>(MIN_YEAR - 1, 1, 1).is_none());
        assert!(flags_ymd::<Federal>(MIN_YEAR, 1, 1).is_some());
        assert!(flags_ymd::<Federal>(MAX_YEAR, 12, 31).is_some());
        assert!(flags_ymd::<Federal>(MAX_YEAR + 1, 1, 1).is_none());
    }

    #[test]
    fn test_range_ymd_can_include_last_supported_day() {
        assert!(
            non_working_days_between_ymd::<Federal>(MAX_YEAR, 12, 31, MAX_YEAR + 1, 1, 1).is_some()
        );
        assert!(
            working_minutes_between_ymd::<Federal>(
                MAX_YEAR,
                12,
                31,
                MAX_YEAR + 1,
                1,
                1,
                WorkWeek::FortyHours,
            )
            .is_some()
        );
    }

    #[test]
    fn test_fact_year_invariants() {
        for year in FIRST_FACT_YEAR..=LAST_FACT_YEAR {
            let mut date = RawDate::from_ymd(year, 1, 1).unwrap();

            loop {
                let flags = calendar::federal_flags_raw(date).value();

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

    #[test]
    fn test_tatarstan_regional_holidays() {
        let r = flags_ymd::<regions::Tatarstan>(2026, 3, 20).unwrap();
        assert!(r.is_fact());
        assert!(r.value().is_holiday());
        assert!(r.value().is_day_off());

        let r = flags_ymd::<regions::Tatarstan>(2026, 3, 21).unwrap();
        assert_eq!(r.value(), DayFlags::EMPTY);

        let r = flags_ymd::<regions::Tatarstan>(2026, 11, 6).unwrap();
        assert!(r.value().is_holiday());
        assert!(r.value().is_day_off());
    }

    #[test]
    fn test_dagestan_multi_day_holidays() {
        for (month, day) in [(3, 19), (3, 20), (5, 27), (5, 28), (5, 29)] {
            let flags = flags_ymd::<regions::Dagestan>(2026, month, day)
                .unwrap()
                .value();
            assert!(flags.is_holiday(), "{month:02}-{day:02}");
            assert!(flags.is_day_off(), "{month:02}-{day:02}");
        }
    }

    #[test]
    fn test_crimea_regional_holidays() {
        for (month, day) in [(3, 18), (4, 13), (6, 1)] {
            let flags = flags_ymd::<regions::Crimea>(2026, month, day)
                .unwrap()
                .value();
            assert!(flags.is_holiday(), "{month:02}-{day:02}");
            assert!(flags.is_day_off(), "{month:02}-{day:02}");
        }
    }

    #[test]
    fn test_tuva_constitution_day_was_added_in_2001() {
        for year in [1999, 2000] {
            let flags = flags_ymd::<regions::Tuva>(year, 5, 6).unwrap().value();
            assert_eq!(flags, DayFlags::EMPTY, "{year}-05-06");
        }

        let flags = flags_ymd::<regions::Tuva>(2001, 5, 6).unwrap().value();
        assert!(flags.is_holiday());
        assert!(flags.is_day_off());
    }

    #[test]
    fn test_irkutsk_oblast_excludes_territorial_sagaalgan() {
        let included = flags_ymd::<regions::IrkutskOblast>(2026, 4, 21)
            .unwrap()
            .value();
        assert!(included.is_holiday());
        assert!(included.is_day_off());

        let excluded = flags_ymd::<regions::IrkutskOblast>(2026, 2, 18)
            .unwrap()
            .value();
        assert_eq!(excluded, DayFlags::EMPTY);
    }

    #[test]
    fn test_regional_overlay_combines_with_federal() {
        let federal = flags_ymd::<Federal>(2026, 3, 20).unwrap().value();
        let regional = flags_ymd::<regions::Tatarstan>(2026, 3, 20)
            .unwrap()
            .value();
        let combined = federal.with_overlay(regional);

        assert!(federal.is_working_day());
        assert!(combined.is_day_off());
        assert!(!combined.is_working_day());
        assert!(combined.is_holiday());
    }

    #[test]
    fn test_regional_out_of_range_is_predict() {
        let fixed = flags_ymd::<regions::Tatarstan>(2027, 8, 30).unwrap();
        assert!(fixed.is_predict());
        assert!(fixed.value().is_holiday());
        assert!(fixed.value().is_day_off());

        let variable = flags_ymd::<regions::Tatarstan>(2027, 3, 20).unwrap();
        assert!(variable.is_predict());
        assert_eq!(variable.value(), DayFlags::EMPTY);
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
            let r = flags::<Federal, _>(date).unwrap();
            assert!(r.is_fact());
            assert!(r.value().is_day_off());
        }

        #[test]
        fn test_is_day_off_with_naive_date() {
            let date = NaiveDate::from_ymd_opt(2026, 1, 9).unwrap();
            assert!(is_day_off::<Federal, _>(date).unwrap().value());
        }

        #[test]
        fn test_predict_with_naive_date() {
            let date = NaiveDate::from_ymd_opt(2027, 1, 1).unwrap();
            let r = flags::<Federal, _>(date).unwrap();
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
                    flags::<Federal, _>(date),
                    flags_ymd::<Federal>(year, month as u8, day as u8)
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
            let r = flags::<Federal, _>(date).unwrap();
            assert!(r.is_fact());
            assert!(r.value().is_day_off());
        }

        #[test]
        fn test_is_day_off_with_time_date() {
            let date = Date::from_calendar_date(2026, Month::January, 9).unwrap();
            assert!(is_day_off::<Federal, _>(date).unwrap().value());
        }

        #[test]
        fn test_predict_with_time_date() {
            let date = Date::from_calendar_date(2027, Month::January, 1).unwrap();
            let r = flags::<Federal, _>(date).unwrap();
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
                assert_eq!(
                    flags::<Federal, _>(date),
                    flags_ymd::<Federal>(year, month.into(), day)
                );
            }
        }
    }
}
