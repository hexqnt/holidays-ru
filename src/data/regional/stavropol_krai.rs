use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Ставропольский край.
pub(crate) const FIRST_YEAR: i32 = 2017;

/// Региональный overlay-календарь: Ставропольский край, 2017 год.
pub(crate) const Y2017: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        Apr: [25],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Ставропольский край, 2018 год.
pub(crate) const Y2018: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        Apr: [17],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Ставропольский край, 2019 год.
pub(crate) const Y2019: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        May: [7],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Ставропольский край, 2020 год.
pub(crate) const Y2020: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        Apr: [28],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Ставропольский край, 2021 год.
pub(crate) const Y2021: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        May: [11],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Ставропольский край, 2022 год.
pub(crate) const Y2022: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        May: [3],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Ставропольский край, 2023 год.
pub(crate) const Y2023: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        Apr: [25],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Ставропольский край, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        May: [14],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Ставропольский край, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        Apr: [29],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Ставропольский край, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        // Закон Ставропольского края от 23.06.2016 N 60-кз: День поминовения усопших (Радоница).
        Apr: [21],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

static YEARS: [YearFact; 10] = [
    Y2017, Y2018, Y2019, Y2020, Y2021, Y2022, Y2023, Y2024, Y2025, Y2026,
];

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
