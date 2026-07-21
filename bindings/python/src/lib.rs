use core::str::FromStr;

use holidays_ru::{DayFlags, Federal, FederalWithRegion, Resolved, WorkWeek, regions};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

type DateParts = (i32, u8, u8);

macro_rules! define_regions {
    ($($variant:ident => ($name:literal, $calendar:ty)),+ $(,)?) => {
        #[derive(Debug, Clone, Copy)]
        enum Region {
            $($variant),+
        }

        const REGION_NAMES: &[&str] = &[$($name),+];

        impl FromStr for Region {
            type Err = ();

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($name => Ok(Self::$variant)),+,
                    _ => Err(()),
                }
            }
        }

        impl Region {
            fn flags_ymd(
                self,
                year: i32,
                month: u8,
                day: u8,
            ) -> Option<Resolved<DayFlags>> {
                match self {
                    $(Self::$variant => holidays_ru::flags_ymd::<FederalWithRegion<$calendar>>(
                        year, month, day,
                    )),+
                }
            }

            fn non_working_days_between(
                self,
                start: DateParts,
                end: DateParts,
            ) -> Option<Resolved<u32>> {
                let (start_year, start_month, start_day) = start;
                let (end_year, end_month, end_day) = end;

                match self {
                    $(Self::$variant => holidays_ru::non_working_days_between_ymd::<
                        FederalWithRegion<$calendar>,
                    >(
                        start_year,
                        start_month,
                        start_day,
                        end_year,
                        end_month,
                        end_day,
                    )),+
                }
            }

            fn working_minutes_between(
                self,
                start: DateParts,
                end: DateParts,
                week: WorkWeek,
            ) -> Option<Resolved<u32>> {
                let (start_year, start_month, start_day) = start;
                let (end_year, end_month, end_day) = end;

                match self {
                    $(Self::$variant => holidays_ru::working_minutes_between_ymd::<
                        FederalWithRegion<$calendar>,
                    >(
                        start_year,
                        start_month,
                        start_day,
                        end_year,
                        end_month,
                        end_day,
                        week,
                    )),+
                }
            }
        }
    };
}

define_regions! {
    Adygea => ("adygea", regions::Adygea),
    AltaiRepublic => ("altai_republic", regions::AltaiRepublic),
    Bashkortostan => ("bashkortostan", regions::Bashkortostan),
    Buryatia => ("buryatia", regions::Buryatia),
    Dagestan => ("dagestan", regions::Dagestan),
    Ingushetia => ("ingushetia", regions::Ingushetia),
    KabardinoBalkaria => ("kabardino_balkaria", regions::KabardinoBalkaria),
    Kalmykia => ("kalmykia", regions::Kalmykia),
    KarachayCherkessia => ("karachay_cherkessia", regions::KarachayCherkessia),
    Crimea => ("crimea", regions::Crimea),
    Mordovia => ("mordovia", regions::Mordovia),
    NorthOssetiaAlania => ("north_ossetia_alania", regions::NorthOssetiaAlania),
    Tatarstan => ("tatarstan", regions::Tatarstan),
    Tuva => ("tuva", regions::Tuva),
    Chechnya => ("chechnya", regions::Chechnya),
    Chuvashia => ("chuvashia", regions::Chuvashia),
    ZabaykalskyKrai => ("zabaykalsky_krai", regions::ZabaykalskyKrai),
    KrasnodarKrai => ("krasnodar_krai", regions::KrasnodarKrai),
    StavropolKrai => ("stavropol_krai", regions::StavropolKrai),
    BelgorodOblast => ("belgorod_oblast", regions::BelgorodOblast),
    BryanskOblast => ("bryansk_oblast", regions::BryanskOblast),
    IrkutskOblast => ("irkutsk_oblast", regions::IrkutskOblast),
    OmskOblast => ("omsk_oblast", regions::OmskOblast),
    PenzaOblast => ("penza_oblast", regions::PenzaOblast),
    SaratovOblast => ("saratov_oblast", regions::SaratovOblast),
}

#[derive(Debug, Clone, Copy)]
enum CalendarSelection {
    Federal,
    Regional(Region),
}

impl CalendarSelection {
    fn parse(region: Option<&str>) -> PyResult<Self> {
        match region {
            None => Ok(Self::Federal),
            Some(value) => Region::from_str(value)
                .map(Self::Regional)
                .map_err(|()| PyValueError::new_err(format!("unsupported region: {value}"))),
        }
    }

    fn flags_ymd(self, year: i32, month: u8, day: u8) -> Option<Resolved<DayFlags>> {
        match self {
            Self::Federal => holidays_ru::flags_ymd::<Federal>(year, month, day),
            Self::Regional(region) => region.flags_ymd(year, month, day),
        }
    }

    fn non_working_days_between(self, start: DateParts, end: DateParts) -> Option<Resolved<u32>> {
        let (start_year, start_month, start_day) = start;
        let (end_year, end_month, end_day) = end;

        match self {
            Self::Federal => holidays_ru::non_working_days_between_ymd::<Federal>(
                start_year,
                start_month,
                start_day,
                end_year,
                end_month,
                end_day,
            ),
            Self::Regional(region) => region.non_working_days_between(start, end),
        }
    }

    fn working_minutes_between(
        self,
        start: DateParts,
        end: DateParts,
        week: WorkWeek,
    ) -> Option<Resolved<u32>> {
        let (start_year, start_month, start_day) = start;
        let (end_year, end_month, end_day) = end;

        match self {
            Self::Federal => holidays_ru::working_minutes_between_ymd::<Federal>(
                start_year,
                start_month,
                start_day,
                end_year,
                end_month,
                end_day,
                week,
            ),
            Self::Regional(region) => region.working_minutes_between(start, end, week),
        }
    }
}

#[inline]
fn into_native<T>(resolved: Resolved<T>) -> (T, bool) {
    let is_official = resolved.is_fact();
    (resolved.value(), is_official)
}

#[pyfunction(signature = (year, month, day, region=None))]
fn _day_info(year: i32, month: u8, day: u8, region: Option<&str>) -> PyResult<(u8, bool)> {
    let calendar = CalendarSelection::parse(region)?;
    let resolved = calendar.flags_ymd(year, month, day).ok_or_else(|| {
        PyValueError::new_err(format!(
            "date must be valid and between {}-01-01 and {}-12-31",
            holidays_ru::MIN_YEAR,
            holidays_ru::MAX_YEAR,
        ))
    })?;
    let (flags, is_official) = into_native(resolved);

    Ok((flags.bits(), is_official))
}

#[pyfunction(signature = (start, end, region=None))]
fn _non_working_days_between(
    start: DateParts,
    end: DateParts,
    region: Option<&str>,
) -> PyResult<(u32, bool)> {
    let calendar = CalendarSelection::parse(region)?;
    let resolved = calendar
        .non_working_days_between(start, end)
        .ok_or_else(|| PyValueError::new_err("invalid or unsupported date range"))?;

    Ok(into_native(resolved))
}

#[pyfunction(signature = (start, end, week, region=None))]
fn _working_minutes_between(
    start: DateParts,
    end: DateParts,
    week: u8,
    region: Option<&str>,
) -> PyResult<(u32, bool)> {
    let week = match week {
        40 => WorkWeek::FortyHours,
        36 => WorkWeek::ThirtySixHours,
        24 => WorkWeek::TwentyFourHours,
        _ => {
            return Err(PyValueError::new_err(
                "work week must be 40, 36, or 24 hours",
            ));
        }
    };
    let calendar = CalendarSelection::parse(region)?;
    let resolved = calendar
        .working_minutes_between(start, end, week)
        .ok_or_else(|| PyValueError::new_err("invalid or unsupported date range"))?;

    Ok(into_native(resolved))
}

#[pymodule]
fn _holidays_ru(module: &Bound<'_, PyModule>) -> PyResult<()> {
    let region_names = PyTuple::new(module.py(), REGION_NAMES)?;

    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    module.add("FIRST_FACT_YEAR", holidays_ru::FIRST_FACT_YEAR)?;
    module.add("LAST_FACT_YEAR", holidays_ru::LAST_FACT_YEAR)?;
    module.add("MIN_YEAR", holidays_ru::MIN_YEAR)?;
    module.add("MAX_YEAR", holidays_ru::MAX_YEAR)?;
    module.add("_REGION_NAMES", region_names)?;
    module.add_function(wrap_pyfunction!(_day_info, module)?)?;
    module.add_function(wrap_pyfunction!(_non_working_days_between, module)?)?;
    module.add_function(wrap_pyfunction!(_working_minutes_between, module)?)?;

    Ok(())
}
