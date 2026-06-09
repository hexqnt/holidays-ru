/// Внутреннее представление даты, независимое от внешних крейтов.
///
/// Используется как общий формат для всех операций библиотеки.
/// Не экспортируется в публичное API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RawDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    /// Порядковый номер дня в году, начиная с 0 (1 января = 0).
    pub ordinal0: u16,
    /// Номер дня недели: 0 = понедельник, ..., 6 = воскресенье.
    pub weekday: u8,
}

impl RawDate {
    /// Создаёт `RawDate` из года, месяца и дня.
    ///
    /// Возвращает `None`, если дата недействительна
    /// (например, 31 февраля, 29 февраля в невисокосный год или год вне
    /// поддерживаемого диапазона `_ymd` API).
    pub(crate) fn from_ymd(year: i32, month: u8, day: u8) -> Option<Self> {
        if !(crate::MIN_YEAR..=crate::MAX_YEAR).contains(&year) {
            return None;
        }

        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return None;
        }

        let max_day = days_in_month(year, month);
        if day > max_day {
            return None;
        }

        let ordinal0 = day_of_year(year, month, day) - 1;
        let weekday = weekday_from_ymd(year, month, day);

        Some(Self {
            year,
            month,
            day,
            ordinal0,
            weekday,
        })
    }

    /// Создаёт `RawDate` без проверки валидности.
    ///
    /// Используется только внутри prediction-алгоритма, где корректность
    /// даты гарантирована предыдущими проверками или жёстко заданными константами.
    pub(crate) const fn from_ymd_unchecked(year: i32, month: u8, day: u8) -> Self {
        let ordinal0 = day_of_year(year, month, day) - 1;
        let weekday = weekday_from_ymd(year, month, day);

        Self {
            year,
            month,
            day,
            ordinal0,
            weekday,
        }
    }

    /// Создаёт `RawDate` из типа, реализующего `CalendarDate`.
    #[cfg(any(feature = "time", feature = "chrono"))]
    #[inline]
    pub(crate) fn from_calendar_date<D: crate::date::CalendarDate>(date: D) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
            ordinal0: date.ordinal0(),
            weekday: date.weekday_number_from_monday(),
        }
    }

    /// Возвращает следующий календарный день.
    ///
    /// Корректно обрабатывает переход через границы месяцев и лет.
    pub(crate) fn next_day(self) -> Self {
        let max_day = days_in_month(self.year, self.month);

        let (year, month, day) = if self.day < max_day {
            (self.year, self.month, self.day + 1)
        } else if self.month < 12 {
            (self.year, self.month + 1, 1)
        } else {
            (self.year + 1, 1, 1)
        };

        Self::from_ymd_unchecked(year, month, day)
    }
}

/// Возвращает количество дней в указанном месяце и году.
#[inline]
pub(crate) const fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Проверяет, является ли год високосным по григорианскому календарю.
#[inline]
pub(crate) const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Возвращает порядковый номер дня в году (1 января = 1, 31 декабря = 365/366).
const fn day_of_year(year: i32, month: u8, day: u8) -> u16 {
    let mut doy = day as u16;
    let mut m: u8 = 1;

    while m < month {
        doy += days_in_month(year, m) as u16;
        m += 1;
    }

    doy
}

/// Вычисляет день недели для заданной даты.
///
/// Возвращает: 0 = понедельник, 1 = вторник, ..., 6 = воскресенье.
///
/// Использует алгоритм Зеллера (Zeller's congruence) для григорианского календаря.
pub(crate) const fn weekday_from_ymd(year: i32, month: u8, day: u8) -> u8 {
    let m = if month < 3 {
        month as i32 + 12
    } else {
        month as i32
    };
    let y = if month < 3 { year - 1 } else { year };

    let c = y / 100;
    let ya = y % 100;

    // Алгоритм Зеллера для григорианского календаря, возвращает 0=суббота, ..., 6=пятница.
    // Формула: h = (day + 13*(m+1)/5 + ya + ya/4 + c/4 - 2*c) mod 7
    let h = (day as i32 + (13 * (m + 1)) / 5 + ya + ya / 4 + c / 4 - 2 * c) % 7;

    // Преобразуем в 0=понедельник, ..., 6=воскресенье.
    // В алгоритме Зеллера: 0=суббота, 1=воскресенье, 2=понедельник, ..., 6=пятница.
    ((h + 5) % 7) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2024, 1), 31); // Январь
        assert_eq!(days_in_month(2024, 2), 29); // Февраль високосного
        assert_eq!(days_in_month(2025, 2), 28); // Февраль невисокосного
        assert_eq!(days_in_month(2024, 4), 30); // Апрель
    }

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2024)); // Делится на 4
        assert!(!is_leap_year(2025));
        assert!(!is_leap_year(1900)); // Делится на 100, но не на 400
        assert!(is_leap_year(2000)); // Делится на 400
    }

    #[test]
    fn test_weekday_known_dates() {
        // 2026-01-01 = четверг (weekday 3)
        assert_eq!(weekday_from_ymd(2026, 1, 1), 3);
        // 2026-01-05 = понедельник (weekday 0)
        assert_eq!(weekday_from_ymd(2026, 1, 5), 0);
        // 2026-01-11 = воскресенье (weekday 6)
        assert_eq!(weekday_from_ymd(2026, 1, 11), 6);
        // 2024-01-01 = понедельник
        assert_eq!(weekday_from_ymd(2024, 1, 1), 0);
        // 2025-01-01 = среда
        assert_eq!(weekday_from_ymd(2025, 1, 1), 2);
    }

    #[test]
    fn test_from_ymd_valid() {
        let d = RawDate::from_ymd(2026, 1, 15).unwrap();
        assert_eq!(d.year, 2026);
        assert_eq!(d.month, 1);
        assert_eq!(d.day, 15);
    }

    #[test]
    fn test_from_ymd_invalid() {
        assert!(RawDate::from_ymd(2026, 2, 31).is_none());
        assert!(RawDate::from_ymd(2026, 13, 1).is_none());
        assert!(RawDate::from_ymd(2026, 0, 1).is_none());
        assert!(RawDate::from_ymd(2026, 1, 0).is_none());
        assert!(RawDate::from_ymd(2025, 2, 29).is_none()); // 2025 не високосный
    }

    #[test]
    fn test_next_day() {
        let d = RawDate::from_ymd(2026, 1, 31).unwrap();
        let next = d.next_day();
        assert_eq!(next.year, 2026);
        assert_eq!(next.month, 2);
        assert_eq!(next.day, 1);

        let d = RawDate::from_ymd(2026, 12, 31).unwrap();
        let next = d.next_day();
        assert_eq!(next.year, 2027);
        assert_eq!(next.month, 1);
        assert_eq!(next.day, 1);
    }

    #[test]
    fn test_ordinal0() {
        let d = RawDate::from_ymd(2026, 1, 1).unwrap();
        assert_eq!(d.ordinal0, 0);

        let d = RawDate::from_ymd(2026, 1, 10).unwrap();
        assert_eq!(d.ordinal0, 9);

        let d = RawDate::from_ymd(2026, 12, 31).unwrap();
        // 2026 не високосный, 365 дней
        assert_eq!(d.ordinal0, 364);
    }
}
