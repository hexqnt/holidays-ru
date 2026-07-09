use crate::Resolved;
use crate::data::YearFact;
use crate::raw_date::RawDate;

use crate::data::months;

/// Первый год, для которого есть региональные данные: Республика Адыгея.
pub(crate) const FIRST_YEAR: i32 = 1995;

/// Региональный overlay-календарь: Республика Адыгея, 1995 год.
pub(crate) const Y1995: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Mar: [2],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 1996 год.
pub(crate) const Y1996: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Feb: [19],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 1997 год.
pub(crate) const Y1997: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Feb: [8],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 1998 год.
pub(crate) const Y1998: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Jan: [29],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 1999 год.
pub(crate) const Y1999: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Jan: [18],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2000 год.
pub(crate) const Y2000: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Jan: [8],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Dec: [27],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2001 год.
pub(crate) const Y2001: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Dec: [16],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2002 год.
pub(crate) const Y2002: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Dec: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2003 год.
pub(crate) const Y2003: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Nov: [25],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2004 год.
pub(crate) const Y2004: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Nov: [14],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2005 год.
pub(crate) const Y2005: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Nov: [3],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2006 год.
pub(crate) const Y2006: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Oct: [5, 23],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2007 год.
pub(crate) const Y2007: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Oct: [5, 13],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2008 год.
pub(crate) const Y2008: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [1, 5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2009 год.
pub(crate) const Y2009: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Sep: [20],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2010 год.
pub(crate) const Y2010: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Sep: [10],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2011 год.
pub(crate) const Y2011: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Aug: [30],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2012 год.
pub(crate) const Y2012: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Aug: [19],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2013 год.
pub(crate) const Y2013: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Aug: [8],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2014 год.
pub(crate) const Y2014: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Jul: [28],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2015 год.
pub(crate) const Y2015: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Jul: [17],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2016 год.
pub(crate) const Y2016: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Jul: [6],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2017 год.
pub(crate) const Y2017: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Jun: [25],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2018 год.
pub(crate) const Y2018: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Jun: [15],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2019 год.
pub(crate) const Y2019: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        Jun: [4],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2020 год.
pub(crate) const Y2020: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        May: [24],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2021 год.
pub(crate) const Y2021: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 06.11.2020 N 384: День поминовения усопших (Радоница).
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        May: [11, 13],
        // Закон Республики Адыгея от 06.11.2020 N 384: Курбан-Байрам.
        Jul: [20],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2022 год.
pub(crate) const Y2022: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        // Закон Республики Адыгея от 06.11.2020 N 384: День поминовения усопших (Радоница).
        May: [2, 3],
        // Закон Республики Адыгея от 06.11.2020 N 384: Курбан-Байрам.
        Jul: [9],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2023 год.
pub(crate) const Y2023: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        // Закон Республики Адыгея от 06.11.2020 N 384: День поминовения усопших (Радоница).
        Apr: [21, 25],
        // Закон Республики Адыгея от 06.11.2020 N 384: Курбан-Байрам.
        Jun: [28],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

/// Региональный overlay-календарь: Республика Адыгея, 2024 год.
pub(crate) const Y2024: YearFact = YearFact {
    holidays: months! {
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        // Указ Главы Республики Адыгея от 09.10.2023 N 133: Ураза-Байрам.
        Apr: [10],
        // Закон Республики Адыгея от 06.11.2020 N 384: День поминовения усопших (Радоница).
        // Указ Главы Республики Адыгея от 09.10.2023 N 133: День поминовения усопших (Радоница).
        May: [14],
        // Закон Республики Адыгея от 06.11.2020 N 384: Курбан-Байрам.
        // Указ Главы Республики Адыгея от 09.10.2023 N 133: Курбан-Байрам.
        Jun: [16],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
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
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        // Указ Главы Республики Адыгея от 31.10.2024 N 139: Ураза-Байрам.
        Mar: [30],
        // Закон Республики Адыгея от 06.11.2020 N 384: День поминовения усопших (Радоница).
        // Указ Главы Республики Адыгея от 31.10.2024 N 139: День поминовения усопших (Радоница).
        Apr: [29],
        // Закон Республики Адыгея от 06.11.2020 N 384: Курбан-Байрам.
        // Указ Главы Республики Адыгея от 31.10.2024 N 139: Курбан-Байрам.
        Jun: [6],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
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
        // Закон Республики Адыгея от 14.02.1995 N 168-1: Ураза-Байрам.
        // Указ Главы Республики Адыгея от 13.10.2025 N 117: Ураза-Байрам.
        Mar: [20],
        // Закон Республики Адыгея от 06.11.2020 N 384: День поминовения усопших (Радоница).
        // Указ Главы Республики Адыгея от 13.10.2025 N 117: День поминовения усопших (Радоница).
        Apr: [21],
        // Закон Республики Адыгея от 06.11.2020 N 384: Курбан-Байрам.
        // Указ Главы Республики Адыгея от 13.10.2025 N 117: Курбан-Байрам.
        May: [27],
        // Закон Республики Адыгея от 14.02.1995 N 168-1: День образования Республики Адыгея.
        Oct: [5],
    },
    extra_days_off: months! {},
    working_days: months! {},
    short_days: months! {},
    transferred_days: months! {},
};

static YEARS: [YearFact; 32] = [
    Y1995, Y1996, Y1997, Y1998, Y1999, Y2000, Y2001, Y2002, Y2003, Y2004, Y2005, Y2006, Y2007,
    Y2008, Y2009, Y2010, Y2011, Y2012, Y2013, Y2014, Y2015, Y2016, Y2017, Y2018, Y2019, Y2020,
    Y2021, Y2022, Y2023, Y2024, Y2025, Y2026,
];

/// Прогноз регионального overlay-календаря по ежегодным фиксированным датам.
static PREDICT: YearFact = YearFact {
    holidays: months! {
        // ежегодный фиксированный праздник.
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
