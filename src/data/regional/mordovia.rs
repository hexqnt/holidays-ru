use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Республика Мордовия.
pub(crate) const FIRST_YEAR: i32 = 2024;

/// Региональный overlay-календарь: Республика Мордовия, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {},
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Мордовия, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {},
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Мордовия, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Республики Мордовия от 08.12.2025 N 473-УГ: День 25-летия канонизации святого праведного воина Феодора Ушакова.
        Aug: [5],
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
