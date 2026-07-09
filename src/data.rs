use crate::DayFlags;
use crate::raw_date::RawDate;

pub(crate) use masks::MonthMasks;
pub(crate) use masks::days;
pub(crate) use months_macro::months;

/// Типы данных для хранения производственного календаря.
///
/// Содержит:
/// - `MonthMasks` — битовые маски дней по месяцам
/// - `YearFact` — полный набор данных за один год
/// - `MonthDay` — пара (месяц, день) для перечислимых структур
/// - `months!` — макрос для читаемого определения данных
mod federal;
mod masks;
mod months_macro;
pub(crate) mod regional;

/// Первый год, для которого есть официальные данные.
pub(crate) const FACT_FIRST_YEAR: i32 = federal::FACT_FIRST_YEAR;

/// Последний год, для которого есть официальные данные.
pub(crate) const FACT_LAST_YEAR: i32 = federal::FACT_LAST_YEAR;

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

/// Фактические данные производственного календаря за один год.
#[derive(Debug, Clone, Copy)]
pub(crate) struct YearFact {
    /// Нерабочие праздничные дни календаря.
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
/// Возвращает официальные данные для указанного года, если они есть.
#[inline]
pub(crate) fn fact_year(year: i32) -> Option<&'static YearFact> {
    federal::fact_year(year)
}

/// Собирает полные `DayFlags` из официальных данных года для конкретной даты.
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

/// Собирает overlay-`DayFlags` из региональных данных года для конкретной даты.
#[inline]
pub(crate) fn flags_from_regional_year_fact(fact: &YearFact, date: RawDate) -> DayFlags {
    let month = date.month;
    let day = date.day;

    let holiday = fact.holidays.contains(month, day);
    let extra_day_off = fact.extra_days_off.contains(month, day);
    let short_day = fact.short_days.contains(month, day);
    let transferred = fact.transferred_days.contains(month, day);

    DayFlags::EMPTY
        .with_if(holiday, DayFlags::HOLIDAY)
        .with_if(holiday || extra_day_off, DayFlags::DAY_OFF)
        .with_if(short_day, DayFlags::SHORT_DAY)
        .with_if(transferred, DayFlags::TRANSFERRED)
}
