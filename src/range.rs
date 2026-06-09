use crate::raw_date::RawDate;
use crate::{DayFlags, Resolved, flags_raw};

/// Норма рабочей недели для расчёта рабочего времени.
///
/// Для рабочих дней используются нормы 8, 7.2 и 4.8 часа соответственно.
/// Сокращённый рабочий день уменьшает норму на 1 час.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkWeek {
    /// 40-часовая рабочая неделя: 8 часов в рабочий день.
    FortyHours,

    /// 36-часовая рабочая неделя: 7 часов 12 минут в рабочий день.
    ThirtySixHours,

    /// 24-часовая рабочая неделя: 4 часа 48 минут в рабочий день.
    TwentyFourHours,
}

impl WorkWeek {
    #[inline]
    pub(crate) const fn daily_minutes(self) -> u32 {
        match self {
            Self::FortyHours => 8 * 60,
            Self::ThirtySixHours => 7 * 60 + 12,
            Self::TwentyFourHours => 4 * 60 + 48,
        }
    }
}

/// Считает нерабочие дни в полуоткрытом диапазоне дат `[start, end)`.
///
/// Правая граница может быть `MAX_YEAR + 1`-01-01, чтобы диапазон мог
/// включать последний поддерживаемый день `MAX_YEAR`-12-31. Если хотя бы один
/// день диапазона рассчитан прогнозом, итоговый результат будет
/// [`Resolved::Predict`].
pub(crate) fn non_working_days_between_raw(start: RawDate, end: RawDate) -> Option<Resolved<u32>> {
    fold_days(start, end, |flags| u32::from(flags.is_day_off()))
}

/// Считает рабочее время в минутах в полуоткрытом диапазоне дат `[start, end)`.
///
/// Нерабочие дни дают 0 минут. Сокращённые рабочие дни уменьшают норму
/// выбранной рабочей недели на 60 минут.
pub(crate) fn working_minutes_between_raw(
    start: RawDate,
    end: RawDate,
    week: WorkWeek,
) -> Option<Resolved<u32>> {
    fold_days(start, end, |flags| working_minutes_for_day(flags, week))
}

#[inline]
fn working_minutes_for_day(flags: DayFlags, week: WorkWeek) -> u32 {
    if !flags.is_working_day() {
        return 0;
    }

    let minutes = week.daily_minutes();

    if flags.is_short_day() {
        minutes.saturating_sub(60)
    } else {
        minutes
    }
}

fn fold_days(
    start: RawDate,
    end: RawDate,
    value: impl Fn(DayFlags) -> u32,
) -> Option<Resolved<u32>> {
    if !start.is_supported() || !end.is_supported_range_end() || start > end {
        return None;
    }

    let mut date = start;
    let mut total = 0u32;
    let mut has_predict = false;

    while date < end {
        let resolved = flags_raw(date);

        has_predict |= resolved.is_predict();
        total = total.saturating_add(value(resolved.value()));
        date = date.next_day();
    }

    if has_predict {
        Some(Resolved::Predict(total))
    } else {
        Some(Resolved::Fact(total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_minutes_for_short_days() {
        let flags = DayFlags::WORKING_DAY.with(DayFlags::SHORT_DAY);

        assert_eq!(working_minutes_for_day(flags, WorkWeek::FortyHours), 420);
        assert_eq!(
            working_minutes_for_day(flags, WorkWeek::ThirtySixHours),
            372
        );
        assert_eq!(
            working_minutes_for_day(flags, WorkWeek::TwentyFourHours),
            228
        );
    }

    #[test]
    fn test_year_totals_2003_2026() {
        for (year, calendar_days, working_days, non_working_days, h40, h36, h24) in [
            (2003, 365, 250, 115, 1992 * 60, 1792 * 60, 1192 * 60),
            (
                2004,
                366,
                251,
                115,
                2004 * 60,
                1803 * 60 + 12,
                1200 * 60 + 48,
            ),
            (
                2005,
                365,
                248,
                117,
                1981 * 60,
                1782 * 60 + 36,
                1187 * 60 + 24,
            ),
            (
                2006,
                365,
                248,
                117,
                1980 * 60,
                1781 * 60 + 36,
                1186 * 60 + 24,
            ),
            (
                2007,
                365,
                249,
                116,
                1986 * 60,
                1786 * 60 + 48,
                1189 * 60 + 12,
            ),
            (2008, 366, 250, 116, 1993 * 60, 1793 * 60, 1193 * 60),
            (
                2009,
                365,
                249,
                116,
                1987 * 60,
                1787 * 60 + 48,
                1190 * 60 + 12,
            ),
            (
                2010,
                365,
                249,
                116,
                1987 * 60,
                1787 * 60 + 48,
                1190 * 60 + 12,
            ),
            (
                2011,
                365,
                248,
                117,
                1981 * 60,
                1782 * 60 + 36,
                1187 * 60 + 24,
            ),
            (
                2012,
                366,
                249,
                117,
                1986 * 60,
                1786 * 60 + 48,
                1189 * 60 + 12,
            ),
            (
                2013,
                365,
                247,
                118,
                1970 * 60,
                1772 * 60 + 24,
                1179 * 60 + 36,
            ),
            (
                2014,
                365,
                247,
                118,
                1970 * 60,
                1772 * 60 + 24,
                1179 * 60 + 36,
            ),
            (
                2015,
                365,
                247,
                118,
                1971 * 60,
                1773 * 60 + 24,
                1180 * 60 + 36,
            ),
            (
                2016,
                366,
                247,
                119,
                1974 * 60,
                1776 * 60 + 24,
                1183 * 60 + 36,
            ),
            (
                2017,
                365,
                247,
                118,
                1973 * 60,
                1775 * 60 + 24,
                1182 * 60 + 36,
            ),
            (
                2018,
                365,
                247,
                118,
                1970 * 60,
                1772 * 60 + 24,
                1179 * 60 + 36,
            ),
            (
                2019,
                365,
                247,
                118,
                1970 * 60,
                1772 * 60 + 24,
                1179 * 60 + 36,
            ),
            (
                2020,
                366,
                248,
                118,
                1979 * 60,
                1780 * 60 + 36,
                1185 * 60 + 24,
            ),
            (
                2021,
                365,
                247,
                118,
                1972 * 60,
                1774 * 60 + 24,
                1181 * 60 + 36,
            ),
            (
                2022,
                365,
                247,
                118,
                1973 * 60,
                1775 * 60 + 24,
                1182 * 60 + 36,
            ),
            (
                2023,
                365,
                247,
                118,
                1973 * 60,
                1775 * 60 + 24,
                1182 * 60 + 36,
            ),
            (
                2024,
                366,
                248,
                118,
                1979 * 60,
                1780 * 60 + 36,
                1185 * 60 + 24,
            ),
            (
                2025,
                365,
                247,
                118,
                1972 * 60,
                1774 * 60 + 24,
                1181 * 60 + 36,
            ),
            (
                2026,
                365,
                247,
                118,
                1972 * 60,
                1774 * 60 + 24,
                1181 * 60 + 36,
            ),
        ] {
            let start = RawDate::from_ymd(year, 1, 1).expect("valid start date");
            let end = RawDate::from_ymd(year + 1, 1, 1).expect("valid end date");

            let non_working = non_working_days_between_raw(start, end).expect("valid range");
            assert_eq!(non_working, Resolved::Fact(non_working_days), "{year}");
            assert_eq!(calendar_days - non_working.value(), working_days, "{year}");

            assert_eq!(
                working_minutes_between_raw(start, end, WorkWeek::FortyHours),
                Some(Resolved::Fact(h40)),
                "{year}"
            );
            assert_eq!(
                working_minutes_between_raw(start, end, WorkWeek::ThirtySixHours),
                Some(Resolved::Fact(h36)),
                "{year}"
            );
            assert_eq!(
                working_minutes_between_raw(start, end, WorkWeek::TwentyFourHours),
                Some(Resolved::Fact(h24)),
                "{year}"
            );
        }
    }

    #[test]
    fn test_empty_range_is_fact_zero() {
        let date = RawDate::from_ymd(2026, 1, 1).expect("valid date");

        assert_eq!(
            non_working_days_between_raw(date, date),
            Some(Resolved::Fact(0))
        );
        assert_eq!(
            working_minutes_between_raw(date, date, WorkWeek::FortyHours),
            Some(Resolved::Fact(0))
        );
    }

    #[test]
    fn test_reversed_range_is_none() {
        let start = RawDate::from_ymd(2026, 1, 2).expect("valid start date");
        let end = RawDate::from_ymd(2026, 1, 1).expect("valid end date");

        assert_eq!(non_working_days_between_raw(start, end), None);
        assert_eq!(
            working_minutes_between_raw(start, end, WorkWeek::FortyHours),
            None
        );
    }

    #[test]
    fn test_mixed_fact_predict_range_is_predict() {
        let start = RawDate::from_ymd(2026, 12, 31).expect("valid start date");
        let end = RawDate::from_ymd(2027, 1, 2).expect("valid end date");

        assert_eq!(
            non_working_days_between_raw(start, end),
            Some(Resolved::Predict(2))
        );
    }

    #[test]
    fn test_range_can_include_last_supported_day() {
        let start = RawDate::from_ymd(crate::MAX_YEAR, 12, 31).expect("valid start date");
        let end = RawDate::from_ymd_unchecked(crate::MAX_YEAR + 1, 1, 1);

        assert!(non_working_days_between_raw(start, end).is_some());
        assert!(working_minutes_between_raw(start, end, WorkWeek::FortyHours).is_some());
    }
}
