/// Типы данных для хранения производственного календаря.
///
/// Содержит:
/// - `MonthMasks` — битовые маски дней по месяцам
/// - `YearFact` — полный набор данных за один год
/// - `MonthDay` — пара (месяц, день) для перечислимых структур
/// - `months!` — макрос для читаемого определения данных
mod masks;
mod months_macro;
mod y1993;
mod y1994;
mod y1995;
mod y1996;
mod y1997;
mod y1998;
mod y1999;
mod y2000;
mod y2001;
mod y2002;
mod y2003;
mod y2004;
mod y2005;
mod y2006;
mod y2007;
mod y2008;
mod y2009;
mod y2010;
mod y2011;
mod y2012;
mod y2013;
mod y2014;
mod y2015;
mod y2016;
mod y2017;
mod y2018;
mod y2019;
mod y2020;
mod y2021;
mod y2022;
mod y2023;
mod y2024;
mod y2025;
mod y2026;

pub(crate) use masks::MonthMasks;
pub(crate) use masks::days;
pub(crate) use months_macro::months;

use crate::DayFlags;
use crate::raw_date::RawDate;

/// Фактические данные производственного календаря за один год.
#[derive(Debug, Clone, Copy)]
pub(crate) struct YearFact {
    /// Федеральные нерабочие праздничные дни (ст. 112 ТК РФ).
    pub holidays: MonthMasks,

    /// Дополнительные выходные дни (переносы с других дат).
    pub extra_days_off: MonthMasks,

    /// Рабочие дни, которые в обычных условиях были бы выходными (рабочие субботы).
    pub working_days: MonthMasks,

    /// Сокращённые рабочие дни (предпраздничные).
    pub short_days: MonthMasks,

    /// Дни, затронутые переносом выходного (и источники, и цели переносов).
    pub transferred_days: MonthMasks,
}

/// Первый год, для которого есть официальные данные.
pub(crate) const FACT_FIRST_YEAR: i32 = 1993;

/// Последний год, для которого есть официальные данные.
pub(crate) const FACT_LAST_YEAR: i32 = 2026;

/// Массив официальных данных по годам, индексируется как `year - FACT_FIRST_YEAR`.
pub(crate) static FACT_YEARS: [YearFact; 34] = [
    y1993::Y1993,
    y1994::Y1994,
    y1995::Y1995,
    y1996::Y1996,
    y1997::Y1997,
    y1998::Y1998,
    y1999::Y1999,
    y2000::Y2000,
    y2001::Y2001,
    y2002::Y2002,
    y2003::Y2003,
    y2004::Y2004,
    y2005::Y2005,
    y2006::Y2006,
    y2007::Y2007,
    y2008::Y2008,
    y2009::Y2009,
    y2010::Y2010,
    y2011::Y2011,
    y2012::Y2012,
    y2013::Y2013,
    y2014::Y2014,
    y2015::Y2015,
    y2016::Y2016,
    y2017::Y2017,
    y2018::Y2018,
    y2019::Y2019,
    y2020::Y2020,
    y2021::Y2021,
    y2022::Y2022,
    y2023::Y2023,
    y2024::Y2024,
    y2025::Y2025,
    y2026::Y2026,
];

/// Возвращает официальные данные для указанного года, если они есть.
#[inline]
pub(crate) fn fact_year(year: i32) -> Option<&'static YearFact> {
    let idx = year.checked_sub(FACT_FIRST_YEAR)? as usize;

    if idx < FACT_YEARS.len() {
        Some(&FACT_YEARS[idx])
    } else {
        None
    }
}

/// Собирает `DayFlags` из официальных данных года для конкретной даты.
#[inline]
pub(crate) fn flags_from_year_fact(fact: &YearFact, date: RawDate) -> DayFlags {
    let month = date.month;
    let day = date.day;

    let weekend = date.weekday >= 5; // 5=Сб, 6=Вс

    let holiday = fact.holidays.contains(month, day);
    let extra_day_off = fact.extra_days_off.contains(month, day);
    let working_day_override = fact.working_days.contains(month, day);
    let short_day = fact.short_days.contains(month, day);
    let transferred = fact.transferred_days.contains(month, day);

    let day_off = !working_day_override && (weekend || holiday || extra_day_off);
    let working_day = !day_off;

    DayFlags::EMPTY
        .with_if(weekend, DayFlags::WEEKEND)
        .with_if(holiday, DayFlags::HOLIDAY)
        .with_if(day_off, DayFlags::DAY_OFF)
        .with_if(working_day, DayFlags::WORKING_DAY)
        .with_if(short_day, DayFlags::SHORT_DAY)
        .with_if(transferred, DayFlags::TRANSFERRED)
}

/// Федеральные нерабочие праздничные дни (ст. 112 ТК РФ).
///
/// Используется в prediction-алгоритме для будущих лет.
pub(crate) const FEDERAL_HOLIDAYS: MonthMasks = months! {
    Jan: [1, 2, 3, 4, 5, 6, 7, 8],
    Feb: [23],
    Mar: [8],
    May: [1, 9],
    Jun: [12],
    Nov: [4],
};

/// Федеральные праздники вне январского блока (1–8 января).
///
/// Используется для предсказания переносов: если такой праздник выпадает
/// на выходной, прогнозируется перенос на ближайший следующий рабочий день.
pub(crate) const NON_JANUARY_HOLIDAYS: [MonthDay; 6] = [
    MonthDay::new(2, 23),
    MonthDay::new(3, 8),
    MonthDay::new(5, 1),
    MonthDay::new(5, 9),
    MonthDay::new(6, 12),
    MonthDay::new(11, 4),
];

/// Пара (месяц, день).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MonthDay {
    pub month: u8,
    pub day: u8,
}

impl MonthDay {
    #[inline]
    pub(crate) const fn new(month: u8, day: u8) -> Self {
        Self { month, day }
    }
}
