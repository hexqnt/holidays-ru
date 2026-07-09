use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Чеченская Республика.
pub(crate) const FIRST_YEAR: i32 = 2003;

/// Региональный overlay-календарь: Чеченская Республика, 2003 год.
pub(crate) const Y2003: YearFact = YearFact {
    holidays: months! {},
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2004 год.
pub(crate) const Y2004: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2005 год.
pub(crate) const Y2005: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2006 год.
pub(crate) const Y2006: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2007 год.
pub(crate) const Y2007: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2008 год.
pub(crate) const Y2008: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2009 год.
pub(crate) const Y2009: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2010 год.
pub(crate) const Y2010: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2011 год.
pub(crate) const Y2011: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2012 год.
pub(crate) const Y2012: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2013 год.
pub(crate) const Y2013: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2014 год.
pub(crate) const Y2014: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2015 год.
pub(crate) const Y2015: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2016 год.
pub(crate) const Y2016: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2017 год.
pub(crate) const Y2017: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2018 год.
pub(crate) const Y2018: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2019 год.
pub(crate) const Y2019: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2020 год.
pub(crate) const Y2020: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2021 год.
pub(crate) const Y2021: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2022 год.
pub(crate) const Y2022: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2023 год.
pub(crate) const Y2023: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [23],
        // Указ Главы Чеченской Республики от 02.04.2024 N 68: Ураза-Байрам.
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [9, 10, 11, 16],
        // Указ Главы Чеченской Республики от 06.06.2024 N 153: Курбан-Байрам.
        Jun: [17, 18],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2025 год.
pub(crate) const Y2025: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        // Указ Главы Чеченской Республики от 24.03.2025 N 47: Ураза-Байрам.
        Mar: [23, 30, 31],
        // Указ Главы Чеченской Республики от 24.03.2025 N 47: Ураза-Байрам.
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [1, 16],
        // Указ Главы Чеченской Республики от 29.05.2025 N 111: Курбан-Байрам.
        Jun: [6, 7, 8],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Чеченская Республика, 2026 год.
pub(crate) const Y2026: YearFact = YearFact {
    holidays: months! {
        // Указ Главы Чеченской Республики от 10.03.2026 N 46: Ураза-Байрам.
        // Указ Главы Администрации Чеченской Республики от 24.03.2003 N 34: День Конституции Чеченской Республики.
        Mar: [19, 20, 21, 23],
        // Указ Президента Чеченской Республики от 04.05.2009 N 155: День мира в Чеченской Республике.
        Apr: [16],
        // Указ Главы Чеченской Республики от 18.05.2026 N 86: Курбан-Байрам.
        May: [27, 28, 29],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

static YEARS: [YearFact; 24] = [
    Y2003, Y2004, Y2005, Y2006, Y2007, Y2008, Y2009, Y2010, Y2011, Y2012, Y2013, Y2014, Y2015,
    Y2016, Y2017, Y2018, Y2019, Y2020, Y2021, Y2022, Y2023, Y2024, Y2025, Y2026,
];

/// Прогноз регионального overlay-календаря по ежегодным фиксированным датам.
static PREDICT: YearFact = YearFact {
    holidays: months! {
        // ежегодный фиксированный праздник.
        Mar: [23],
        // ежегодный фиксированный праздник.
        Apr: [16],
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
