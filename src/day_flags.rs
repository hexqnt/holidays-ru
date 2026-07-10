/// Компактное представление свойств дня через битовые флаги.
///
/// Каждый бит обозначает одно свойство:
/// - `WEEKEND` — суббота или воскресенье
/// - `HOLIDAY` — нерабочий праздничный день выбранного календаря
/// - `DAY_OFF` — день является выходным
/// - `WORKING_DAY` — день является рабочим
/// - `SHORT_DAY` — сокращённый рабочий день
/// - `TRANSFERRED` — день затронут переносом выходного
///
/// # Алгебра календарей
///
/// Над `DayFlags` определены две операции для совмещения календарей
/// (например, федерального и регионального, или графиков разных сотрудников):
///
/// | Оператор | Семантика |
/// |---|---|
/// | `a + b` | Консервативное сложение: выходной побеждает. Если хотя бы один календарь говорит «выходной» — день выходной |
/// | `a * b` | Либеральное умножение: рабочий побеждает. День выходной, только если оба говорят «выходной» |
///
/// Нейтральные элементы:
/// - [`EMPTY`](Self::EMPTY) — нейтральный для `+` (пустой календарь)
///
/// Для корректных календарных значений обе операции идемпотентны:
/// `a + a = a`, `a * a = a`.
///
/// # Примеры
///
/// ```rust
/// use holidays_ru::DayFlags;
///
/// let flags = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
/// assert!(flags.is_holiday());
/// assert!(flags.is_day_off());
/// assert!(!flags.is_working_day());
///
/// // Алгебра: федеральный + региональный календарь
/// let federal = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
/// let regional = DayFlags::WORKING_DAY; // в регионе работают
///
/// // Когда общий выходной? (консервативно: выходной побеждает)
/// assert!((federal + regional).is_day_off());
///
/// // Когда оба работают? (либерально: рабочий побеждает)
/// assert!((federal * regional).is_working_day());
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DayFlags(u8);

impl DayFlags {
    /// Выходной день недели (суббота или воскресенье).
    pub const WEEKEND: Self = Self(1 << 0);

    /// Нерабочий праздничный день выбранного календаря.
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
    ///
    /// Нейтральный элемент для [`Add`](core::ops::Add): `a + EMPTY = a`.
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

    /// Объединяет полный календарь с overlay-календарём.
    ///
    /// Если overlay помечает дату как нерабочую, флаги
    /// [`WORKING_DAY`](Self::WORKING_DAY) и [`SHORT_DAY`](Self::SHORT_DAY)
    /// из базового календаря снимаются.
    #[inline]
    #[must_use]
    pub const fn with_overlay(self, overlay: Self) -> Self {
        let mut bits = self.0 | overlay.0;

        if overlay.is_day_off() {
            bits &= !(Self::WORKING_DAY.0 | Self::SHORT_DAY.0);
        }

        Self(bits)
    }

    /// Является ли день выходным днём недели (суббота или воскресенье).
    #[inline]
    #[must_use]
    pub const fn is_weekend(self) -> bool {
        self.0 & Self::WEEKEND.0 != 0
    }

    /// Является ли день нерабочим праздничным днём выбранного календаря.
    #[inline]
    #[must_use]
    pub const fn is_holiday(self) -> bool {
        self.0 & Self::HOLIDAY.0 != 0
    }

    /// Является ли день выходным (нерабочим).
    #[inline]
    #[must_use]
    pub const fn is_day_off(self) -> bool {
        self.0 & Self::DAY_OFF.0 != 0
    }

    /// Является ли день рабочим.
    #[inline]
    #[must_use]
    pub const fn is_working_day(self) -> bool {
        self.0 & Self::WORKING_DAY.0 != 0
    }

    /// Является ли день сокращённым рабочим днём.
    #[inline]
    #[must_use]
    pub const fn is_short_day(self) -> bool {
        self.0 & Self::SHORT_DAY.0 != 0
    }

    /// Затронут ли день переносом выходного.
    #[inline]
    #[must_use]
    pub const fn is_transferred(self) -> bool {
        self.0 & Self::TRANSFERRED.0 != 0
    }

    /// Возвращает сырое значение битовой маски.
    #[inline]
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Создаёт `DayFlags` из сырой битовой маски.
    ///
    /// Не выполняет проверку корректности комбинации флагов.
    #[inline]
    #[must_use]
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }
}

// ---------------------------------------------------------------------------
// Алгебра календарей
// ---------------------------------------------------------------------------

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

impl core::ops::Add for DayFlags {
    type Output = Self;

    /// Консервативное сложение календарей: выходной побеждает.
    ///
    /// Если хотя бы один операнд говорит «выходной» — результат выходной.
    /// Если никто не говорит «выходной» и хотя бы один говорит «рабочий» —
    /// результат рабочий.
    ///
    /// [`EMPTY`](Self::EMPTY) — нейтральный элемент для корректных календарных
    /// значений: `a + EMPTY = a`.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use holidays_ru::DayFlags;
    ///
    /// // Суббота (выходной) + рабочий день → выходной (выходной побеждает)
    /// let weekend_off = DayFlags::WEEKEND.with(DayFlags::DAY_OFF);
    /// let result = weekend_off + DayFlags::WORKING_DAY;
    /// assert!(result.is_day_off());
    /// assert!(result.is_weekend());
    ///
    /// // Рабочий + рабочий → рабочий
    /// let result = DayFlags::WORKING_DAY + DayFlags::WORKING_DAY;
    /// assert!(result.is_working_day());
    /// ```
    fn add(self, rhs: Self) -> Self {
        let mut result = Self::EMPTY;

        // Выходной, если хотя бы один выходной
        if self.is_day_off() || rhs.is_day_off() {
            result = result.with(Self::DAY_OFF);
        }

        // Праздник, если хотя бы один праздник
        if self.is_holiday() || rhs.is_holiday() {
            result = result.with(Self::HOLIDAY);
        }

        // Выходной день недели, если хотя бы у одного
        if self.is_weekend() || rhs.is_weekend() {
            result = result.with(Self::WEEKEND);
        }

        // Перенос, если хотя бы у одного
        if self.is_transferred() || rhs.is_transferred() {
            result = result.with(Self::TRANSFERRED);
        }

        // Рабочий, если никто не говорит «выходной» и хотя бы один рабочий
        if !result.is_day_off() && (self.is_working_day() || rhs.is_working_day()) {
            result = result.with(Self::WORKING_DAY);

            // Сокращённый, только если результат рабочий и хотя бы один сокращённый
            if self.is_short_day() || rhs.is_short_day() {
                result = result.with(Self::SHORT_DAY);
            }
        }

        result
    }
}

impl core::ops::Mul for DayFlags {
    type Output = Self;

    /// Либеральное умножение календарей: рабочий день побеждает.
    ///
    /// День выходной, только если оба операнда говорят «выходной».
    /// Если хотя бы один говорит «рабочий» — результат рабочий.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use holidays_ru::DayFlags;
    ///
    /// // Суббота (выходной) * рабочий день → рабочий (работа побеждает)
    /// let weekend_off = DayFlags::WEEKEND.with(DayFlags::DAY_OFF);
    /// let result = weekend_off * DayFlags::WORKING_DAY;
    /// assert!(result.is_working_day());
    ///
    /// // Суббота (выходной) * праздник (выходной) → выходной (оба отдыхают)
    /// let holiday_off = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
    /// let result = weekend_off * holiday_off;
    /// assert!(result.is_day_off());
    /// ```
    fn mul(self, rhs: Self) -> Self {
        let mut result = Self::EMPTY;

        // Рабочий, если хотя бы один рабочий
        if self.is_working_day() || rhs.is_working_day() {
            result = result.with(Self::WORKING_DAY);

            // Сокращённый, если результат рабочий и хотя бы один сокращённый
            if self.is_short_day() || rhs.is_short_day() {
                result = result.with(Self::SHORT_DAY);
            }
        }

        // Праздник, только если оба праздничные
        if self.is_holiday() && rhs.is_holiday() {
            result = result.with(Self::HOLIDAY);
        }

        // Выходной день недели, только если оба
        if self.is_weekend() && rhs.is_weekend() {
            result = result.with(Self::WEEKEND);
        }

        // Перенос, только если оба
        if self.is_transferred() && rhs.is_transferred() {
            result = result.with(Self::TRANSFERRED);
        }

        // Выходной, только если оба выходные и результат не рабочий
        if self.is_day_off() && rhs.is_day_off() && !result.is_working_day() {
            result = result.with(Self::DAY_OFF);
        }

        result
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

#[cfg(test)]
mod algebra_tests {
    use super::*;

    const ALL_BITS: core::ops::Range<u8> = 0..64;

    fn valid_calendar_flags(flags: DayFlags) -> bool {
        !(flags.is_day_off() && flags.is_working_day())
            && (!flags.is_short_day() || flags.is_working_day())
            && (!flags.is_holiday() || flags.is_day_off())
    }

    // -----------------------------------------------------------------------
    // Add (+)
    // -----------------------------------------------------------------------

    #[test]
    fn add_working_plus_working_is_working() {
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::WORKING_DAY;
        let r = a + b;
        assert!(r.is_working_day());
        assert!(!r.is_day_off());
    }

    #[test]
    fn add_working_plus_weekend_is_day_off() {
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::WEEKEND.with(DayFlags::DAY_OFF);
        let r = a + b;
        assert!(r.is_day_off());
        assert!(r.is_weekend());
    }

    #[test]
    fn add_holiday_wins_over_working() {
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        let r = a + b;
        assert!(r.is_day_off());
        assert!(r.is_holiday());
        assert!(!r.is_working_day());
    }

    #[test]
    fn add_short_plus_working_is_short() {
        let a = DayFlags::WORKING_DAY.with(DayFlags::SHORT_DAY);
        let b = DayFlags::WORKING_DAY;
        let r = a + b;
        assert!(r.is_working_day());
        assert!(r.is_short_day());
    }

    #[test]
    fn add_transferred_is_union() {
        let a = DayFlags::WORKING_DAY.with(DayFlags::TRANSFERRED);
        let b = DayFlags::WORKING_DAY;
        let r = a + b;
        assert!(r.is_transferred());
    }

    #[test]
    fn add_idempotent() {
        let a = DayFlags::HOLIDAY
            .with(DayFlags::DAY_OFF)
            .with(DayFlags::TRANSFERRED);
        assert_eq!(a + a, a);
    }

    #[test]
    fn add_commutative() {
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn add_empty_is_neutral() {
        let a = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        assert_eq!(a + DayFlags::EMPTY, a);
        assert_eq!(DayFlags::EMPTY + a, a);
    }

    #[test]
    fn add_short_lost_when_result_is_day_off() {
        let a = DayFlags::WORKING_DAY.with(DayFlags::SHORT_DAY);
        let b = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        let r = a + b;
        assert!(r.is_day_off());
        assert!(!r.is_short_day()); // сокращённый теряется, т.к. день выходной
    }

    // -----------------------------------------------------------------------
    // Mul (*)
    // -----------------------------------------------------------------------

    #[test]
    fn mul_working_times_weekend_is_working() {
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::WEEKEND;
        let r = a * b;
        assert!(r.is_working_day());
        assert!(!r.is_day_off());
    }

    #[test]
    fn mul_off_times_off_is_off() {
        // Оба выходные, но типы разные: праздник × обычный выходной → выходной
        let a = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        let b = DayFlags::WEEKEND.with(DayFlags::DAY_OFF);
        let r = a * b;
        // Оба выходные → результат выходной
        assert!(r.is_day_off());
        // Но HOLIDAY только у a, поэтому в пересечении его нет
        assert!(!r.is_holiday());
        // WEEKEND только у b, поэтому в пересечении его нет
        assert!(!r.is_weekend());
    }

    #[test]
    fn mul_working_times_off_is_working() {
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::DAY_OFF;
        let r = a * b;
        assert!(r.is_working_day());
        assert!(!r.is_day_off());
    }

    #[test]
    fn mul_short_preserved() {
        let a = DayFlags::WORKING_DAY.with(DayFlags::SHORT_DAY);
        let b = DayFlags::WORKING_DAY;
        let r = a * b;
        assert!(r.is_working_day());
        assert!(r.is_short_day());
    }

    #[test]
    fn mul_idempotent() {
        let a = DayFlags::WORKING_DAY
            .with(DayFlags::SHORT_DAY)
            .with(DayFlags::TRANSFERRED);
        assert_eq!(a * a, a);
    }

    #[test]
    fn mul_commutative() {
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        assert_eq!(a * b, b * a);
    }

    #[test]
    fn mul_transferred_is_intersection() {
        let a = DayFlags::WORKING_DAY.with(DayFlags::TRANSFERRED);
        let b = DayFlags::WORKING_DAY;
        let r = a * b;
        assert!(!r.is_transferred());
    }

    #[test]
    fn operations_are_closed_for_valid_calendar_flags() {
        for a in ALL_BITS
            .map(DayFlags::from_bits)
            .filter(|f| valid_calendar_flags(*f))
        {
            for b in ALL_BITS
                .map(DayFlags::from_bits)
                .filter(|f| valid_calendar_flags(*f))
            {
                assert!(valid_calendar_flags(a + b), "{a:?} + {b:?}");
                assert!(valid_calendar_flags(a * b), "{a:?} * {b:?}");
            }
        }
    }

    #[test]
    fn operations_are_algebraic_for_valid_calendar_flags() {
        let valid_flags = ALL_BITS
            .map(DayFlags::from_bits)
            .filter(|f| valid_calendar_flags(*f))
            .collect::<Vec<_>>();

        for &a in &valid_flags {
            assert_eq!(a + a, a, "add idempotent for {a:?}");
            assert_eq!(a * a, a, "mul idempotent for {a:?}");
            assert_eq!(a + DayFlags::EMPTY, a, "add right neutral for {a:?}");
            assert_eq!(DayFlags::EMPTY + a, a, "add left neutral for {a:?}");

            for &b in &valid_flags {
                assert_eq!(a + b, b + a, "add commutative for {a:?}, {b:?}");
                assert_eq!(a * b, b * a, "mul commutative for {a:?}, {b:?}");

                for &c in &valid_flags {
                    assert_eq!((a + b) + c, a + (b + c), "add associative");
                    assert_eq!((a * b) * c, a * (b * c), "mul associative");
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // Distributive law (документирует недистрибутивность алгебры)
    // -----------------------------------------------------------------------

    #[test]
    fn distributivity_does_not_hold() {
        // Алгебра не дистрибутивна: a * (b + c) ≠ a*b + a*c в общем случае.
        // Это связано с тем, что + и * разрешают конфликты рабочести
        // (выходной побеждает в +, рабочий побеждает в *).
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::WORKING_DAY.with(DayFlags::SHORT_DAY);
        let c = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        // a * (b + c): b+c даёт выходной (c побеждает),
        //   затем a * выходной = рабочий (a побеждает в *)
        let left = a * (b + c);
        // a*b + a*c: a*b = рабочий+сокращённый, a*c = рабочий,
        //   их сумма = рабочий+сокращённый
        let right = a * b + a * c;
        // left = WORKING_DAY, right = WORKING_DAY|SHORT_DAY → не равны
        assert_ne!(left, right);
        assert_eq!(left, DayFlags::WORKING_DAY);
        assert_eq!(right, DayFlags::WORKING_DAY.with(DayFlags::SHORT_DAY));
    }

    #[test]
    fn add_associative() {
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        let c = DayFlags::WEEKEND;
        assert_eq!((a + b) + c, a + (b + c));
    }

    #[test]
    fn mul_associative() {
        let a = DayFlags::WORKING_DAY;
        let b = DayFlags::WORKING_DAY.with(DayFlags::SHORT_DAY);
        let c = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        assert_eq!((a * b) * c, a * (b * c));
    }

    // -----------------------------------------------------------------------
    // Practical scenarios
    // -----------------------------------------------------------------------

    #[test]
    fn federal_plus_regional_common_day_off() {
        // Федеральный праздник + региональный рабочий → общий выходной
        let federal = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        let regional = DayFlags::WORKING_DAY;
        let common = federal + regional;
        assert!(common.is_day_off());
        assert!(common.is_holiday());
    }

    #[test]
    fn overlay_day_off_clears_working_and_short_day() {
        let federal = DayFlags::WORKING_DAY.with(DayFlags::SHORT_DAY);
        let regional = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);

        let combined = federal.with_overlay(regional);

        assert!(combined.is_day_off());
        assert!(combined.is_holiday());
        assert!(!combined.is_working_day());
        assert!(!combined.is_short_day());
    }

    #[test]
    fn federal_times_regional_overlap_working() {
        // Когда оба работают? Пересечение.
        let federal = DayFlags::WORKING_DAY;
        let regional = DayFlags::HOLIDAY.with(DayFlags::DAY_OFF);
        let overlap = federal * regional;
        assert!(overlap.is_working_day());
    }

    #[test]
    fn two_employees_both_work() {
        // Сотрудник А: рабочая суббота
        // Сотрудник Б: обычный рабочий день
        // Когда оба на работе? (* — рабочий побеждает)
        let emp_a = DayFlags::WORKING_DAY
            .with(DayFlags::TRANSFERRED)
            .with(DayFlags::WEEKEND);
        let emp_b = DayFlags::WORKING_DAY;
        let both = emp_a * emp_b;
        // Оба работают — результат рабочий
        assert!(both.is_working_day());
        // TRANSFERRED только у А, в пересечении его нет
        assert!(!both.is_transferred());
        // WEEKEND только у А, в пересечении его нет
        assert!(!both.is_weekend());
    }

    #[test]
    fn two_employees_both_rest() {
        // Сотрудник А: отпуск (day off)
        // Сотрудник Б: выходной
        // Когда оба отдыхают? (+ — выходной побеждает)
        let emp_a = DayFlags::DAY_OFF;
        let emp_b = DayFlags::WEEKEND.with(DayFlags::DAY_OFF);
        let both = emp_a + emp_b;
        assert!(both.is_day_off());
        assert!(both.is_weekend());
    }
}
