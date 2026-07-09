use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Саратовская область.
pub(crate) const FIRST_YEAR: i32 = 2012;

/// Региональный overlay-календарь: Саратовская область, 2012 год.
pub(crate) const Y2012: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        Apr: [24],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2013 год.
pub(crate) const Y2013: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        May: [14],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2014 год.
pub(crate) const Y2014: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        Apr: [29],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2015 год.
pub(crate) const Y2015: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        Apr: [21],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2016 год.
pub(crate) const Y2016: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        May: [10],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2017 год.
pub(crate) const Y2017: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        Apr: [25],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2018 год.
pub(crate) const Y2018: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        Apr: [17],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2019 год.
pub(crate) const Y2019: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        May: [7],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2020 год.
pub(crate) const Y2020: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        Apr: [28],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2021 год.
pub(crate) const Y2021: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        May: [11],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2022 год.
pub(crate) const Y2022: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        May: [3],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2023 год.
pub(crate) const Y2023: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        Apr: [25],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        May: [14],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        Apr: [29],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Саратовская область, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        // Закон Саратовской области от 22.03.2012 N 26-ЗСО: Радоница - день особого поминовения усопших.
        Apr: [21],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

static YEARS: [YearFact; 15] = [
    Y2012, Y2013, Y2014, Y2015, Y2016, Y2017, Y2018, Y2019, Y2020, Y2021, Y2022, Y2023, Y2024,
    Y2025, Y2026,
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
