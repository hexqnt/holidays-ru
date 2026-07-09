use super::YearFact;

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

/// Первый год, для которого есть официальные федеральные данные.
pub(crate) const FACT_FIRST_YEAR: i32 = 1993;

/// Последний год, для которого есть официальные федеральные данные.
pub(crate) const FACT_LAST_YEAR: i32 = 2026;

/// Массив официальных федеральных данных по годам.
///
/// Индексируется как `year - FACT_FIRST_YEAR`.
static FACT_YEARS: [YearFact; 34] = [
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

/// Возвращает официальные федеральные данные для указанного года, если они есть.
#[inline]
pub(crate) fn fact_year(year: i32) -> Option<&'static YearFact> {
    let idx = usize::try_from(year.checked_sub(FACT_FIRST_YEAR)?).ok()?;

    FACT_YEARS.get(idx)
}
