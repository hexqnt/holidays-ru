use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Республика Калмыкия.
pub(crate) const FIRST_YEAR: i32 = 2024;

/// Региональный overlay-календарь: Республика Калмыкия, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        Feb: [10],
        May: [23],
        Jul: [5],
        Dec: [25, 28],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Калмыкия, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        Feb: [28],
        Jun: [11],
        Jul: [5],
        Dec: [14, 28],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Калмыкия, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        Feb: [18],
        May: [31],
        Jul: [5],
        Dec: [28],
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
        Jul: [5],
        Dec: [28],
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
