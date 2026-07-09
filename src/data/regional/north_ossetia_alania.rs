use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Республика Северная Осетия - Алания.
pub(crate) const FIRST_YEAR: i32 = 2018;

/// Региональный overlay-календарь: Республика Северная Осетия - Алания, 2018 год.
pub(crate) const Y2018: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Северная Осетия - Алания от 02.10.2018 N 61-РЗ: первый понедельник праздника Уастырджи.
        Nov: [19],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Северная Осетия - Алания, 2019 год.
pub(crate) const Y2019: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Северная Осетия - Алания от 02.10.2018 N 61-РЗ: первый понедельник праздника Уастырджи.
        Nov: [18],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Северная Осетия - Алания, 2020 год.
pub(crate) const Y2020: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Северная Осетия - Алания от 02.10.2018 N 61-РЗ: первый понедельник праздника Уастырджи.
        Nov: [23],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Северная Осетия - Алания, 2021 год.
pub(crate) const Y2021: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Северная Осетия - Алания от 02.10.2018 N 61-РЗ: первый понедельник праздника Уастырджи.
        Nov: [22],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Северная Осетия - Алания, 2022 год.
pub(crate) const Y2022: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Северная Осетия - Алания от 02.10.2018 N 61-РЗ: первый понедельник праздника Уастырджи.
        Nov: [21],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Северная Осетия - Алания, 2023 год.
pub(crate) const Y2023: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Северная Осетия - Алания от 02.10.2018 N 61-РЗ: первый понедельник праздника Уастырджи.
        Nov: [20],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Северная Осетия - Алания, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Северная Осетия - Алания от 02.10.2018 N 61-РЗ: первый понедельник праздника Уастырджи.
        Nov: [18],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Северная Осетия - Алания, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Республики Северная Осетия-Алания от 12.11.2025 N 453: дополнительный день праздника Уастырджи.
        // Закон Республики Северная Осетия - Алания от 02.10.2018 N 61-РЗ: первый понедельник праздника Уастырджи.
        Nov: [17, 24],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Северная Осетия - Алания, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Северная Осетия - Алания от 02.10.2018 N 61-РЗ: первый понедельник праздника Уастырджи.
        Nov: [23],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

static YEARS: [YearFact; 9] = [
    Y2018, Y2019, Y2020, Y2021, Y2022, Y2023, Y2024, Y2025, Y2026,
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
