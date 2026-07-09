use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Белгородская область.
pub(crate) const FIRST_YEAR: i32 = 2020;

/// Региональный overlay-календарь: Белгородская область, 2020 год.
pub(crate) const Y2020: YearFact = YearFact {
    holidays: months! {
        // Закон Белгородской области от 30.04.2020 N 462: День Прохоровского поля - Третьего ратного поля России.
        Jul: [12],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Белгородская область, 2021 год.
pub(crate) const Y2021: YearFact = YearFact {
    holidays: months! {
        // Закон Белгородской области от 30.04.2020 N 462: День Прохоровского поля - Третьего ратного поля России.
        Jul: [12],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Белгородская область, 2022 год.
pub(crate) const Y2022: YearFact = YearFact {
    holidays: months! {
        // Закон Белгородской области от 30.04.2020 N 462: День Прохоровского поля - Третьего ратного поля России.
        Jul: [12],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Белгородская область, 2023 год.
pub(crate) const Y2023: YearFact = YearFact {
    holidays: months! {
        // Закон Белгородской области от 30.04.2020 N 462: День Прохоровского поля - Третьего ратного поля России.
        Jul: [12],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Белгородская область, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        // Закон Белгородской области от 30.04.2020 N 462: День Прохоровского поля - Третьего ратного поля России.
        Jul: [12],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Белгородская область, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        // Закон Белгородской области от 30.04.2020 N 462: День Прохоровского поля - Третьего ратного поля России.
        Jul: [12],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Белгородская область, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        // Закон Белгородской области от 30.04.2020 N 462: День Прохоровского поля - Третьего ратного поля России.
        Jul: [12],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

static YEARS: [YearFact; 7] = [Y2020, Y2021, Y2022, Y2023, Y2024, Y2025, Y2026];

/// Прогноз регионального overlay-календаря по ежегодным фиксированным датам.
static PREDICT: YearFact = YearFact {
    holidays: months! {
        // ежегодный фиксированный праздник.
        Jul: [12],
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
