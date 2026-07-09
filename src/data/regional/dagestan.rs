use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

/// Первый год, для которого есть региональные данные: Республика Дагестан.
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
        Jul: [26],
        Sep: [15],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Дагестан, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        Apr: [10, 11, 12],
        Jun: [17],
        Jul: [26],
        Sep: [15],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Дагестан, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        Mar: [31],
        Apr: [1, 29],
        Jun: [6],
        Jul: [26],
        Sep: [15],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Дагестан, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        Mar: [19, 20],
        Apr: [13, 21],
        May: [27, 28, 29],
        Jul: [26],
        Sep: [15],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};
