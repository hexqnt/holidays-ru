//! Региональные overlay-календари на основе `info/regional_holidays.md`.
//!
//! Источник сохранён 09.07.2026. Даты с ограничением `<*>` и даты,
//! применимые только к отдельным территориям субъекта, здесь не включены.
use crate::raw_date::RawDate;
use crate::{DayFlags, Resolved};

use super::{YearFact, flags_from_regional_year_fact};

pub(crate) mod adygea;
pub(crate) mod altai_republic;
pub(crate) mod bashkortostan;
pub(crate) mod belgorod_oblast;
pub(crate) mod bryansk_oblast;
pub(crate) mod buryatia;
pub(crate) mod chechnya;
pub(crate) mod chuvashia;
pub(crate) mod crimea;
pub(crate) mod dagestan;
pub(crate) mod ingushetia;
pub(crate) mod irkutsk_oblast;
pub(crate) mod kabardino_balkaria;
pub(crate) mod kalmykia;
pub(crate) mod karachay_cherkessia;
pub(crate) mod krasnodar_krai;
pub(crate) mod mordovia;
pub(crate) mod north_ossetia_alania;
pub(crate) mod omsk_oblast;
pub(crate) mod penza_oblast;
pub(crate) mod saratov_oblast;
pub(crate) mod stavropol_krai;
pub(crate) mod tatarstan;
pub(crate) mod tuva;
pub(crate) mod zabaykalsky_krai;

#[inline]
fn resolve(
    date: RawDate,
    first_year: i32,
    years: &'static [YearFact],
    predict: &'static YearFact,
) -> Resolved<DayFlags> {
    let fact = date
        .year
        .checked_sub(first_year)
        .and_then(|idx| usize::try_from(idx).ok())
        .and_then(|idx| years.get(idx));

    match fact {
        Some(fact) => Resolved::Fact(flags_from_regional_year_fact(fact, date)),
        None => Resolved::Predict(flags_from_regional_year_fact(predict, date)),
    }
}
