use crate::DayFlags;
use crate::data::{fact_year, flags_from_year_fact};
use crate::raw_date::RawDate;

/// Пытается получить официальные флаги дня из таблиц производственного календаря.
///
/// Возвращает `Some(flags)`, если для года даты есть официальные данные,
/// и `None` в противном случае (тогда следует использовать `predict`).
///
/// Сложность: O(1) — индексация массива и несколько битовых проверок.
#[inline]
pub(crate) fn flags(date: RawDate) -> Option<DayFlags> {
    let fact = fact_year(date.year)?;
    Some(flags_from_year_fact(fact, date))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_official_2026_jan1_is_holiday_day_off() {
        let d = RawDate::from_ymd(2026, 1, 1).unwrap();
        let flags = flags(d).unwrap();
        assert!(flags.is_holiday());
        assert!(flags.is_day_off());
        assert!(!flags.is_working_day());
        // 1 января 2026 — четверг, не weekend
        assert!(!flags.is_weekend());
    }

    #[test]
    fn test_official_2026_jan9_extra_day_off() {
        let d = RawDate::from_ymd(2026, 1, 9).unwrap();
        let flags = flags(d).unwrap();
        assert!(flags.is_day_off());
        assert!(!flags.is_holiday());
        assert!(flags.is_transferred());
    }

    #[test]
    fn test_official_2026_jan11_weekend() {
        let d = RawDate::from_ymd(2026, 1, 11).unwrap();
        let flags = flags(d).unwrap();
        assert!(flags.is_weekend());
        assert!(flags.is_day_off());
        assert!(!flags.is_holiday());
    }

    #[test]
    fn test_official_2026_jan12_working_day() {
        let d = RawDate::from_ymd(2026, 1, 12).unwrap();
        let flags = flags(d).unwrap();
        assert!(flags.is_working_day());
        assert!(!flags.is_day_off());
    }

    #[test]
    fn test_official_2026_feb20_not_short_day() {
        // 20 февраля 2026 — пятница, но 21-22 февраля — выходные.
        let d = RawDate::from_ymd(2026, 2, 20).unwrap();
        let flags = flags(d).unwrap();
        assert!(flags.is_working_day());
        assert!(!flags.is_short_day());
    }

    #[test]
    fn test_official_2026_nov3_short_day() {
        // 3 ноября 2026 — вторник, короткий день перед 4 ноября.
        let d = RawDate::from_ymd(2026, 11, 3).unwrap();
        let flags = flags(d).unwrap();
        assert!(flags.is_working_day());
        assert!(flags.is_short_day());
    }

    #[test]
    fn test_official_year_out_of_range_is_none() {
        let d = RawDate::from_ymd(1992, 1, 1).unwrap();
        assert!(flags(d).is_none());

        let d = RawDate::from_ymd(2027, 1, 1).unwrap();
        assert!(flags(d).is_none());
    }

    #[test]
    fn test_official_1993_jan4_extra_day_off() {
        // 4 января 1993 — доп. выходной за 2 января, выпавшее на субботу.
        let d = RawDate::from_ymd(1993, 1, 4).unwrap();
        let f = flags(d).unwrap();
        assert!(f.is_day_off());
        assert!(!f.is_holiday());
        assert!(f.is_transferred());
    }

    #[test]
    fn test_official_1999_dec31_short_day() {
        // 31 декабря 1999 — сокращённый рабочий день перед 1 января.
        let d = RawDate::from_ymd(1999, 12, 31).unwrap();
        let f = flags(d).unwrap();
        assert!(f.is_working_day());
        assert!(f.is_short_day());
    }

    // -------------------------------------------------------------------
    // Проверка эпох: 2010–2012 (старые праздники) vs 2013+ (новые)
    // -------------------------------------------------------------------

    #[test]
    fn test_2010_jan5_is_holiday_jan6_is_not() {
        // 2010: Новогодние каникулы 1–5 января. 6 января — доп. выходной.
        let d5 = RawDate::from_ymd(2010, 1, 5).unwrap();
        let f5 = flags(d5).unwrap();
        assert!(f5.is_holiday());

        let d6 = RawDate::from_ymd(2010, 1, 6).unwrap();
        let f6 = flags(d6).unwrap();
        assert!(
            !f6.is_holiday(),
            "6 января 2010 — не праздник, а доп. выходной"
        );
        assert!(f6.is_day_off());
        assert!(f6.is_transferred());
    }

    #[test]
    fn test_2013_jan6_is_holiday() {
        // С 2013: Новогодние каникулы 1–6 и 8 января.
        let d = RawDate::from_ymd(2013, 1, 6).unwrap();
        let f = flags(d).unwrap();
        assert!(f.is_holiday(), "6 января 2013 — праздничный день");
    }

    #[test]
    fn test_2024_apr27_working_saturday() {
        // 27 апреля 2024 — рабочая суббота, короткий день перед 1 мая.
        let d = RawDate::from_ymd(2024, 4, 27).unwrap();
        let f = flags(d).unwrap();
        assert!(f.is_working_day());
        assert!(f.is_short_day());
        assert!(f.is_transferred());
    }

    #[test]
    fn test_2024_dec28_working_saturday() {
        // 28 декабря 2024 — рабочая суббота.
        let d = RawDate::from_ymd(2024, 12, 28).unwrap();
        let f = flags(d).unwrap();
        assert!(f.is_working_day());
        assert!(f.is_transferred());
    }

    #[test]
    fn test_2026_mar6_not_short_day() {
        // 6 марта 2026 — пятница, но 7 марта — выходной.
        let d = RawDate::from_ymd(2026, 3, 6).unwrap();
        let f = flags(d).unwrap();
        assert!(f.is_working_day());
        assert!(!f.is_short_day());
    }

    #[test]
    fn test_2026_mar9_extra_day_off() {
        // 9 марта 2026 — понедельник, доп. выходной (перенос с 8 марта).
        let d = RawDate::from_ymd(2026, 3, 9).unwrap();
        let f = flags(d).unwrap();
        assert!(f.is_day_off());
        assert!(!f.is_holiday());
        assert!(f.is_transferred());
    }
}
