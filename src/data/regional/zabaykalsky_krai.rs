use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Забайкальский край.
pub(crate) const FIRST_YEAR: i32 = 2024;

/// Региональный overlay-календарь: Забайкальский край, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        // Постановление Губернатора Забайкальского края от 19.09.2023 N 67: Сагаалган.
        Feb: [10],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Забайкальский край, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        // Постановление Губернатора Забайкальского края от 12.09.2024 N 82: Сагаалган.
        Mar: [1],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Забайкальский край, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        // Постановление Губернатора Забайкальского края от 05.09.2025 N 99: Сагаалган.
        Feb: [18],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

static YEARS: [YearFact; 3] = [Y2024, Y2025, Y2026];

/// Прогноз регионального overlay-календаря по ежегодным фиксированным датам.
static PREDICT: YearFact = YearFact {
    holidays: months! {},
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

#[inline]
pub(crate) fn flags(date: RawDate) -> Resolved<crate::DayFlags> {
    super::resolve(date, FIRST_YEAR, &YEARS, &PREDICT)
}
