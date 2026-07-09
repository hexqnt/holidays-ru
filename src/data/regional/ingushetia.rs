use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Республика Ингушетия.
pub(crate) const FIRST_YEAR: i32 = 2024;

/// Региональный overlay-календарь: Республика Ингушетия, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Республики Ингушетия от 04.04.2024 N 47: Ид аль-Фитр (Мархаш).
        Apr: [10, 11, 12],
        // Указ Главы Республики Ингушетия от 06.06.2024 N 67: Ид аль-Адха (Г1урба).
        Jun: [17],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Ингушетия, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Республики Ингушетия от 26.03.2025 N 46: Ид аль-Фитр (Мархаш).
        Mar: [31],
        // Указ Главы Республики Ингушетия от 26.03.2025 N 46: Ид аль-Фитр (Мархаш).
        Apr: [1],
        // Указ Главы Республики Ингушетия от 30.05.2025 N 84: Ид аль-Адха (Г1урба).
        Jun: [6, 7],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Ингушетия, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Республики Ингушетия от 16.03.2026 N 38: Ид аль-Фитр (Мархаш).
        Mar: [19, 20, 21],
        // Указ Главы Республики Ингушетия от 20.05.2026 N 84: Ид аль-Адха (Г1урба).
        May: [27, 28, 29],
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
