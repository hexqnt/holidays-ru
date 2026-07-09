use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

/// Первый год, для которого есть региональные данные: Республика Тыва.
pub(crate) const FIRST_YEAR: i32 = 2024;

static YEARS: [YearFact; 3] = [Y2024, Y2025, Y2026];

#[inline]
pub(crate) fn flags(date: RawDate) -> Resolved<crate::DayFlags> {
    super::resolve(date, FIRST_YEAR, &YEARS, &PREDICT)
}

use crate::data::months;

/// Прогноз регионального overlay-календаря по ежегодным фиксированным датам.
static PREDICT: YearFact = YearFact {
    holidays: months! {
        Aug: [15],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Тыва, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        Feb: [10],
        May: [6],
        Jul: [19],
        Aug: [15],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Тыва, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        Mar: [1],
        May: [6],
        Jul: [18],
        Aug: [15],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Тыва, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        Feb: [18],
        May: [8],
        Jul: [24],
        Aug: [15],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};
