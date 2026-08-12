//! Нативная часть Julia-биндингов `holidays-ru`.
//!
//! FFI использует только примитивные `isbits`-типы. Результат упакован в
//! `u64`: младшие 32 бита содержат значение, бит 32 отмечает официальные
//! данные, бит 33 — корректный результат.

use holidays_ru::Resolved;
use holidays_ru_bindings_common::{
    CalendarSelection, DateParts, REGIONS, Region, work_week_from_hours,
};
use jlrs::prelude::*;

const VALUE_MASK: u64 = u32::MAX as u64;
const OFFICIAL_BIT: u64 = 1 << 32;
const VALID_BIT: u64 = 1 << 33;
const CALENDAR_FEDERAL_CODE: u8 = CalendarSelection::FEDERAL_CODE;

/// Первый год с официальными федеральными данными.
pub const FIRST_FACT_YEAR: i32 = holidays_ru::FIRST_FACT_YEAR;

/// Последний год с официальными федеральными данными.
pub const LAST_FACT_YEAR: i32 = holidays_ru::LAST_FACT_YEAR;

/// Минимальный поддерживаемый год.
pub const MIN_YEAR: i32 = holidays_ru::MIN_YEAR;

/// Максимальный поддерживаемый год.
pub const MAX_YEAR: i32 = holidays_ru::MAX_YEAR;

const REGION_COUNT: u8 = REGIONS.len() as u8;
const REGION_ADYGEA: u8 = Region::Adygea.code();
const REGION_ALTAI_REPUBLIC: u8 = Region::AltaiRepublic.code();
const REGION_BASHKORTOSTAN: u8 = Region::Bashkortostan.code();
const REGION_BURYATIA: u8 = Region::Buryatia.code();
const REGION_DAGESTAN: u8 = Region::Dagestan.code();
const REGION_INGUSHETIA: u8 = Region::Ingushetia.code();
const REGION_KABARDINO_BALKARIA: u8 = Region::KabardinoBalkaria.code();
const REGION_KALMYKIA: u8 = Region::Kalmykia.code();
const REGION_KARACHAY_CHERKESSIA: u8 = Region::KarachayCherkessia.code();
const REGION_CRIMEA: u8 = Region::Crimea.code();
const REGION_MORDOVIA: u8 = Region::Mordovia.code();
const REGION_NORTH_OSSETIA_ALANIA: u8 = Region::NorthOssetiaAlania.code();
const REGION_TATARSTAN: u8 = Region::Tatarstan.code();
const REGION_TUVA: u8 = Region::Tuva.code();
const REGION_CHECHNYA: u8 = Region::Chechnya.code();
const REGION_CHUVASHIA: u8 = Region::Chuvashia.code();
const REGION_ZABAYKALSKY_KRAI: u8 = Region::ZabaykalskyKrai.code();
const REGION_KRASNODAR_KRAI: u8 = Region::KrasnodarKrai.code();
const REGION_STAVROPOL_KRAI: u8 = Region::StavropolKrai.code();
const REGION_BELGOROD_OBLAST: u8 = Region::BelgorodOblast.code();
const REGION_BRYANSK_OBLAST: u8 = Region::BryanskOblast.code();
const REGION_IRKUTSK_OBLAST: u8 = Region::IrkutskOblast.code();
const REGION_OMSK_OBLAST: u8 = Region::OmskOblast.code();
const REGION_PENZA_OBLAST: u8 = Region::PenzaOblast.code();
const REGION_SARATOV_OBLAST: u8 = Region::SaratovOblast.code();
const FLAG_WEEKEND: u8 = holidays_ru::DayFlags::WEEKEND.bits();
const FLAG_HOLIDAY: u8 = holidays_ru::DayFlags::HOLIDAY.bits();
const FLAG_DAY_OFF: u8 = holidays_ru::DayFlags::DAY_OFF.bits();
const FLAG_WORKING_DAY: u8 = holidays_ru::DayFlags::WORKING_DAY.bits();
const FLAG_SHORT_DAY: u8 = holidays_ru::DayFlags::SHORT_DAY.bits();
const FLAG_TRANSFERRED: u8 = holidays_ru::DayFlags::TRANSFERRED.bits();

#[inline]
fn pack(resolved: Option<Resolved<u32>>) -> u64 {
    match resolved {
        None => 0,
        Some(resolved) => {
            let source = if resolved.is_fact() { OFFICIAL_BIT } else { 0 };
            VALID_BIT | source | (u64::from(resolved.value()) & VALUE_MASK)
        }
    }
}

#[inline]
fn calendar(code: u8) -> Option<CalendarSelection> {
    CalendarSelection::from_code(code)
}

fn native_day_info(year: i32, month: u8, day: u8, calendar_code: u8) -> u64 {
    let resolved = calendar(calendar_code)
        .and_then(|calendar| calendar.flags_ymd(year, month, day))
        .map(|resolved| resolved.map(|flags| u32::from(flags.bits())));

    pack(resolved)
}

#[allow(clippy::too_many_arguments)]
fn native_non_working_days_between(
    start_year: i32,
    start_month: u8,
    start_day: u8,
    end_year: i32,
    end_month: u8,
    end_day: u8,
    calendar_code: u8,
) -> u64 {
    let start: DateParts = (start_year, start_month, start_day);
    let end: DateParts = (end_year, end_month, end_day);
    let resolved =
        calendar(calendar_code).and_then(|calendar| calendar.non_working_days_between(start, end));

    pack(resolved)
}

#[allow(clippy::too_many_arguments)]
fn native_working_minutes_between(
    start_year: i32,
    start_month: u8,
    start_day: u8,
    end_year: i32,
    end_month: u8,
    end_day: u8,
    week_hours: u8,
    calendar_code: u8,
) -> u64 {
    let start: DateParts = (start_year, start_month, start_day);
    let end: DateParts = (end_year, end_month, end_day);
    let resolved = match (calendar(calendar_code), work_week_from_hours(week_hours)) {
        (Some(calendar), Some(week)) => calendar.working_minutes_between(start, end, week),
        _ => None,
    };

    pack(resolved)
}

julia_module! {
    become holidays_ru_julia_init;

    const FIRST_FACT_YEAR: i32;
    const LAST_FACT_YEAR: i32;
    const MIN_YEAR: i32;
    const MAX_YEAR: i32;

    const REGION_COUNT: u8 as _REGION_COUNT;
    const REGION_ADYGEA: u8 as _REGION_ADYGEA;
    const REGION_ALTAI_REPUBLIC: u8 as _REGION_ALTAI_REPUBLIC;
    const REGION_BASHKORTOSTAN: u8 as _REGION_BASHKORTOSTAN;
    const REGION_BURYATIA: u8 as _REGION_BURYATIA;
    const REGION_DAGESTAN: u8 as _REGION_DAGESTAN;
    const REGION_INGUSHETIA: u8 as _REGION_INGUSHETIA;
    const REGION_KABARDINO_BALKARIA: u8 as _REGION_KABARDINO_BALKARIA;
    const REGION_KALMYKIA: u8 as _REGION_KALMYKIA;
    const REGION_KARACHAY_CHERKESSIA: u8 as _REGION_KARACHAY_CHERKESSIA;
    const REGION_CRIMEA: u8 as _REGION_CRIMEA;
    const REGION_MORDOVIA: u8 as _REGION_MORDOVIA;
    const REGION_NORTH_OSSETIA_ALANIA: u8 as _REGION_NORTH_OSSETIA_ALANIA;
    const REGION_TATARSTAN: u8 as _REGION_TATARSTAN;
    const REGION_TUVA: u8 as _REGION_TUVA;
    const REGION_CHECHNYA: u8 as _REGION_CHECHNYA;
    const REGION_CHUVASHIA: u8 as _REGION_CHUVASHIA;
    const REGION_ZABAYKALSKY_KRAI: u8 as _REGION_ZABAYKALSKY_KRAI;
    const REGION_KRASNODAR_KRAI: u8 as _REGION_KRASNODAR_KRAI;
    const REGION_STAVROPOL_KRAI: u8 as _REGION_STAVROPOL_KRAI;
    const REGION_BELGOROD_OBLAST: u8 as _REGION_BELGOROD_OBLAST;
    const REGION_BRYANSK_OBLAST: u8 as _REGION_BRYANSK_OBLAST;
    const REGION_IRKUTSK_OBLAST: u8 as _REGION_IRKUTSK_OBLAST;
    const REGION_OMSK_OBLAST: u8 as _REGION_OMSK_OBLAST;
    const REGION_PENZA_OBLAST: u8 as _REGION_PENZA_OBLAST;
    const REGION_SARATOV_OBLAST: u8 as _REGION_SARATOV_OBLAST;

    const VALUE_MASK: u64 as _VALUE_MASK;
    const OFFICIAL_BIT: u64 as _OFFICIAL_BIT;
    const VALID_BIT: u64 as _VALID_BIT;
    const CALENDAR_FEDERAL_CODE: u8 as _CALENDAR_FEDERAL_CODE;
    const FLAG_WEEKEND: u8 as _FLAG_WEEKEND;
    const FLAG_HOLIDAY: u8 as _FLAG_HOLIDAY;
    const FLAG_DAY_OFF: u8 as _FLAG_DAY_OFF;
    const FLAG_WORKING_DAY: u8 as _FLAG_WORKING_DAY;
    const FLAG_SHORT_DAY: u8 as _FLAG_SHORT_DAY;
    const FLAG_TRANSFERRED: u8 as _FLAG_TRANSFERRED;

    #[gc_safe]
    fn native_day_info(
        year: i32,
        month: u8,
        day: u8,
        calendar_code: u8,
    ) -> u64 as _native_day_info;

    #[gc_safe]
    fn native_non_working_days_between(
        start_year: i32,
        start_month: u8,
        start_day: u8,
        end_year: i32,
        end_month: u8,
        end_day: u8,
        calendar_code: u8,
    ) -> u64 as _native_non_working_days_between;

    #[gc_safe]
    fn native_working_minutes_between(
        start_year: i32,
        start_month: u8,
        start_day: u8,
        end_year: i32,
        end_month: u8,
        end_day: u8,
        week_hours: u8,
        calendar_code: u8,
    ) -> u64 as _native_working_minutes_between;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_official_day_flags() {
        let packed = native_day_info(2026, 1, 9, CalendarSelection::FEDERAL_CODE);

        assert_ne!(packed & VALID_BIT, 0);
        assert_ne!(packed & OFFICIAL_BIT, 0);
        assert_ne!(packed & u64::from(holidays_ru::DayFlags::DAY_OFF.bits()), 0);
    }

    #[test]
    fn rejects_invalid_calendar_code() {
        assert_eq!(native_day_info(2026, 1, 9, u8::MAX), 0);
    }

    #[test]
    fn rejects_invalid_work_week() {
        let packed =
            native_working_minutes_between(2026, 1, 12, 2026, 1, 13, 8, CALENDAR_FEDERAL_CODE);

        assert_eq!(packed, 0);
    }

    #[test]
    fn regional_range_uses_full_calendar() {
        let packed =
            native_non_working_days_between(2026, 11, 6, 2026, 11, 7, Region::Tatarstan.code());

        assert_eq!(packed & VALUE_MASK, 1);
        assert_ne!(packed & VALID_BIT, 0);
    }
}
