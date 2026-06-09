/// Компактное представление свойств дня через битовые флаги.
///
/// Каждый бит обозначает одно свойство:
/// - `WEEKEND` — суббота или воскресенье
/// - `HOLIDAY` — федеральный нерабочий праздничный день (ст. 112 ТК РФ)
/// - `DAY_OFF` — день является выходным
/// - `WORKING_DAY` — день является рабочим
/// - `SHORT_DAY` — сокращённый рабочий день
/// - `TRANSFERRED` — день затронут переносом выходного
///
/// # Пример
///
/// ```rust
/// use holidays_ru::DayFlags;
///
/// let flags = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
/// assert!(flags.is_holiday());
/// assert!(flags.is_day_off());
/// assert!(!flags.is_working_day());
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DayFlags(u8);

impl DayFlags {
    /// Выходной день недели (суббота или воскресенье).
    pub const WEEKEND: Self = Self(1 << 0);

    /// Федеральный нерабочий праздничный день.
    pub const HOLIDAY: Self = Self(1 << 1);

    /// День является выходным (нерабочим).
    pub const DAY_OFF: Self = Self(1 << 2);

    /// День является рабочим.
    pub const WORKING_DAY: Self = Self(1 << 3);

    /// Сокращённый рабочий день (предпраздничный).
    pub const SHORT_DAY: Self = Self(1 << 4);

    /// День затронут переносом выходного.
    pub const TRANSFERRED: Self = Self(1 << 5);

    /// Пустой набор флагов (ни одно свойство не установлено).
    pub const EMPTY: Self = Self(0);

    /// Добавляет флаги `other` к текущему набору.
    #[inline]
    #[must_use]
    pub const fn with(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Добавляет `other`, если `condition` истинно.
    #[inline]
    #[must_use]
    pub const fn with_if(self, condition: bool, other: Self) -> Self {
        Self(self.0 | (other.0 & (condition as u8).wrapping_neg()))
    }

    /// Является ли день выходным днём недели (суббота или воскресенье).
    #[inline]
    pub const fn is_weekend(self) -> bool {
        self.0 & Self::WEEKEND.0 != 0
    }

    /// Является ли день федеральным нерабочим праздничным днём.
    #[inline]
    pub const fn is_holiday(self) -> bool {
        self.0 & Self::HOLIDAY.0 != 0
    }

    /// Является ли день выходным (нерабочим).
    #[inline]
    pub const fn is_day_off(self) -> bool {
        self.0 & Self::DAY_OFF.0 != 0
    }

    /// Является ли день рабочим.
    #[inline]
    pub const fn is_working_day(self) -> bool {
        self.0 & Self::WORKING_DAY.0 != 0
    }

    /// Является ли день сокращённым рабочим днём.
    #[inline]
    pub const fn is_short_day(self) -> bool {
        self.0 & Self::SHORT_DAY.0 != 0
    }

    /// Затронут ли день переносом выходного.
    #[inline]
    pub const fn is_transferred(self) -> bool {
        self.0 & Self::TRANSFERRED.0 != 0
    }

    /// Возвращает сырое значение битовой маски.
    #[inline]
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Создаёт `DayFlags` из сырой битовой маски.
    ///
    /// Не выполняет проверку корректности комбинации флагов.
    #[inline]
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for DayFlags {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DayFlags {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <u8 as serde::Deserialize>::deserialize(deserializer).map(Self)
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    #[test]
    fn test_serde_json_as_bits() {
        let flags = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        let json = serde_json::to_string(&flags).unwrap();

        assert_eq!(json, "6");
        assert_eq!(serde_json::from_str::<DayFlags>(&json).unwrap(), flags);
    }
}
