use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Республика Адыгея.
pub(crate) const FIRST_YEAR: i32 = 2024;

/// Региональный overlay-календарь: Республика Адыгея, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        Apr: [10],
        May: [14],
        Jun: [16],
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        Mar: [30],
        Apr: [29],
        Jun: [6],
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        Mar: [20],
        Apr: [21],
        May: [27],
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};
static YEARS: [YearFact; 3] = [Y2024, Y2025, Y2026];

/// Прогноз регионального overlay-календаря по ежегодным фиксированным датам.
static PREDICT: YearFact = YearFact {
    holidays: months! {
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

#[inline]
pub(crate) fn flags(date: RawDate) -> Resolved<crate::DayFlags> {
    super::resolve(date, FIRST_YEAR, &YEARS, &PREDICT)
}
