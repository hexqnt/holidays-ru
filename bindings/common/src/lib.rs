//! Общая динамическая диспетчеризация для языковых биндингов `holidays-ru`.
//!
//! Основной крейт намеренно использует типизированные marker-календари. Этот
//! внутренний адаптер сводит их к небольшим enum для динамических языков.

use core::fmt;
use core::str::FromStr;

use holidays_ru::{DayFlags, Federal, FederalWithRegion, Resolved, WorkWeek, regions};

/// Дата в виде `(year, month, day)`.
pub type DateParts = (i32, u8, u8);

macro_rules! define_regions {
    ($($variant:ident = $code:literal => ($name:literal, $calendar:ty)),+ $(,)?) => {
        /// Региональный календарь, объединяемый с федеральным календарём.
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Region {
            $($variant = $code),+
        }

        /// Все поддерживаемые регионы в стабильном порядке их числовых кодов.
        pub const REGIONS: &[Region] = &[$(Region::$variant),+];

        /// Строковые идентификаторы всех поддерживаемых регионов.
        pub const REGION_NAMES: &[&str] = &[$($name),+];

        impl Region {
            /// Возвращает стабильный числовой код региона для FFI.
            #[inline]
            #[must_use]
            pub const fn code(self) -> u8 {
                self as u8
            }

            /// Возвращает строковый идентификатор региона.
            #[inline]
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name),+
                }
            }

            /// Создаёт регион из стабильного числового FFI-кода.
            #[inline]
            #[must_use]
            pub const fn from_code(code: u8) -> Option<Self> {
                match code {
                    $($code => Some(Self::$variant)),+,
                    _ => None,
                }
            }

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

        impl FromStr for Region {
            type Err = UnsupportedRegion;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($name => Ok(Self::$variant)),+,
                    _ => Err(UnsupportedRegion::new(value)),
                }
            }
        }
    };
}

define_regions! {
    Adygea = 1 => ("adygea", regions::Adygea),
    AltaiRepublic = 2 => ("altai_republic", regions::AltaiRepublic),
    Bashkortostan = 3 => ("bashkortostan", regions::Bashkortostan),
    Buryatia = 4 => ("buryatia", regions::Buryatia),
    Dagestan = 5 => ("dagestan", regions::Dagestan),
    Ingushetia = 6 => ("ingushetia", regions::Ingushetia),
    KabardinoBalkaria = 7 => ("kabardino_balkaria", regions::KabardinoBalkaria),
    Kalmykia = 8 => ("kalmykia", regions::Kalmykia),
    KarachayCherkessia = 9 => ("karachay_cherkessia", regions::KarachayCherkessia),
    Crimea = 10 => ("crimea", regions::Crimea),
    Mordovia = 11 => ("mordovia", regions::Mordovia),
    NorthOssetiaAlania = 12 => ("north_ossetia_alania", regions::NorthOssetiaAlania),
    Tatarstan = 13 => ("tatarstan", regions::Tatarstan),
    Tuva = 14 => ("tuva", regions::Tuva),
    Chechnya = 15 => ("chechnya", regions::Chechnya),
    Chuvashia = 16 => ("chuvashia", regions::Chuvashia),
    ZabaykalskyKrai = 17 => ("zabaykalsky_krai", regions::ZabaykalskyKrai),
    KrasnodarKrai = 18 => ("krasnodar_krai", regions::KrasnodarKrai),
    StavropolKrai = 19 => ("stavropol_krai", regions::StavropolKrai),
    BelgorodOblast = 20 => ("belgorod_oblast", regions::BelgorodOblast),
    BryanskOblast = 21 => ("bryansk_oblast", regions::BryanskOblast),
    IrkutskOblast = 22 => ("irkutsk_oblast", regions::IrkutskOblast),
    OmskOblast = 23 => ("omsk_oblast", regions::OmskOblast),
    PenzaOblast = 24 => ("penza_oblast", regions::PenzaOblast),
    SaratovOblast = 25 => ("saratov_oblast", regions::SaratovOblast),
}

/// Ошибка разбора неизвестного строкового идентификатора региона.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedRegion {
    name: String,
}

impl UnsupportedRegion {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }

    /// Возвращает неизвестный строковый идентификатор региона.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for UnsupportedRegion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unsupported region: {}", self.name)
    }
}

impl std::error::Error for UnsupportedRegion {}

/// Динамически выбранный федеральный или федерально-региональный календарь.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CalendarSelection {
    /// Федеральный производственный календарь.
    Federal,

    /// Федеральный календарь с региональными исключениями.
    Regional(Region),
}

impl CalendarSelection {
    /// FFI-код федерального календаря.
    pub const FEDERAL_CODE: u8 = 0;

    /// Разбирает необязательный строковый идентификатор региона.
    ///
    /// `None` выбирает федеральный календарь. Неизвестный идентификатор
    /// возвращается в [`UnsupportedRegion`].
    pub fn parse(region: Option<&str>) -> Result<Self, UnsupportedRegion> {
        match region {
            None => Ok(Self::Federal),
            Some(value) => Region::from_str(value).map(Self::Regional),
        }
    }

    /// Создаёт календарь из стабильного FFI-кода: `0` означает федеральный.
    #[must_use]
    pub const fn from_code(code: u8) -> Option<Self> {
        if code == Self::FEDERAL_CODE {
            Some(Self::Federal)
        } else {
            match Region::from_code(code) {
                Some(region) => Some(Self::Regional(region)),
                None => None,
            }
        }
    }

    /// Возвращает флаги дня или `None` для некорректной либо неподдерживаемой даты.
    #[must_use]
    pub fn flags_ymd(self, year: i32, month: u8, day: u8) -> Option<Resolved<DayFlags>> {
        match self {
            Self::Federal => holidays_ru::flags_ymd::<Federal>(year, month, day),
            Self::Regional(region) => region.flags_ymd(year, month, day),
        }
    }

    /// Считает нерабочие дни в полуоткрытом диапазоне `[start, end)`.
    ///
    /// Возвращает `None`, если границы некорректны, не поддерживаются или идут
    /// в обратном порядке.
    #[must_use]
    pub fn non_working_days_between(
        self,
        start: DateParts,
        end: DateParts,
    ) -> Option<Resolved<u32>> {
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

    /// Считает рабочее время в минутах в полуоткрытом диапазоне `[start, end)`.
    ///
    /// Возвращает `None`, если границы некорректны, не поддерживаются или идут
    /// в обратном порядке.
    #[must_use]
    pub fn working_minutes_between(
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

/// Преобразует количество часов рабочей недели в тип ядра.
///
/// Поддерживаются нормы 40, 36 и 24 часа; для остальных значений возвращается
/// `None`.
#[inline]
#[must_use]
pub const fn work_week_from_hours(hours: u8) -> Option<WorkWeek> {
    match hours {
        40 => Some(WorkWeek::FortyHours),
        36 => Some(WorkWeek::ThirtySixHours),
        24 => Some(WorkWeek::TwentyFourHours),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_codes_and_names_round_trip() {
        assert_eq!(REGIONS.len(), REGION_NAMES.len());

        for (index, region) in REGIONS.iter().copied().enumerate() {
            assert_eq!(usize::from(region.code()), index + 1);
            assert_eq!(Region::from_code(region.code()), Some(region));
            assert_eq!(Region::from_str(region.name()), Ok(region));
        }
    }

    #[test]
    fn calendar_codes_include_federal_and_reject_unknown_values() {
        assert_eq!(
            CalendarSelection::from_code(CalendarSelection::FEDERAL_CODE),
            Some(CalendarSelection::Federal)
        );
        assert_eq!(CalendarSelection::from_code(u8::MAX), None);
    }

    #[test]
    fn unsupported_region_error_contains_the_name() {
        let result = Region::from_str("unknown");

        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.name(), "unknown");
            assert_eq!(error.to_string(), "unsupported region: unknown");
        }
    }

    #[test]
    fn regional_selection_combines_with_federal_calendar() {
        let calendar = CalendarSelection::Regional(Region::Tatarstan);
        let flags = calendar.flags_ymd(2026, 11, 6).map(Resolved::value);

        assert!(flags.is_some_and(DayFlags::is_day_off));
    }
}
