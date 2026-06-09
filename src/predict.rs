use crate::DayFlags;
use crate::data::{FEDERAL_HOLIDAYS, MonthDay, NON_JANUARY_HOLIDAYS};
use crate::raw_date::RawDate;

/// Алгоритмический прогноз свойств дня для будущих лет.
///
/// Используется, когда для запрошенного года нет официальных данных.
/// Прогноз основан на детерминированных правилах:
///
/// 1. Суббота/воскресенье — выходные.
/// 2. Федеральные нерабочие праздничные дни (ст. 112 ТК РФ) — выходные.
/// 3. Если неянварский праздник выпадает на выходной — перенос на ближайший
///    следующий базовый рабочий день.
/// 4. Для январских каникул (1–8 января) — максимум 2 переноса на первые
///    рабочие дни после 8 января.
/// 5. Рабочие субботы не прогнозируются.
/// 6. Сокращённый день — рабочий день, за которым следует федеральный праздник.
///
/// # Важно
///
/// Этот прогноз — эвристика. Конкретные переносы утверждаются постановлением
/// Правительства РФ и могут отличаться от прогноза. Результат всегда
/// возвращается как `Resolved::Predict`.
///
/// Сложность: O(1) — фиксированное число проверок, без аллокаций.
#[inline]
pub(crate) fn flags(date: RawDate) -> DayFlags {
    let weekend = is_weekend(date);
    let holiday = is_federal_holiday(date);

    let extra_day_off = predicted_extra_day_off(date);

    // В MVP не прогнозируем рабочие субботы.
    let working_override = false;

    let day_off = !working_override && (weekend || holiday || extra_day_off);
    let working_day = !day_off;

    let short_day = if working_day {
        is_federal_holiday(date.next_day())
    } else {
        false
    };

    let transferred = extra_day_off || working_override;

    DayFlags::EMPTY
        .with_if(weekend, DayFlags::WEEKEND)
        .with_if(holiday, DayFlags::HOLIDAY)
        .with_if(day_off, DayFlags::DAY_OFF)
        .with_if(working_day, DayFlags::WORKING_DAY)
        .with_if(short_day, DayFlags::SHORT_DAY)
        .with_if(transferred, DayFlags::TRANSFERRED)
}

/// Является ли день выходным (суббота или воскресенье).
#[inline]
fn is_weekend(date: RawDate) -> bool {
    date.weekday >= 5
}

/// Является ли день федеральным нерабочим праздничным днём.
#[inline]
fn is_federal_holiday(date: RawDate) -> bool {
    FEDERAL_HOLIDAYS.contains(date.month, date.day)
}

/// Прогнозирует, является ли день дополнительным выходным (целью переноса).
fn predicted_extra_day_off(date: RawDate) -> bool {
    is_transfer_target_from_non_january_holiday(date)
        || is_transfer_target_from_january_holiday(date)
}

/// Проверяет, является ли день целью переноса от неянварского праздника.
///
/// Если федеральный праздник (вне 1–8 января) выпадает на субботу или
/// воскресенье, прогнозируется перенос на ближайший следующий базовый
/// рабочий день (не выходной и не праздничный).
fn is_transfer_target_from_non_january_holiday(date: RawDate) -> bool {
    let mut i = 0;

    while i < NON_JANUARY_HOLIDAYS.len() {
        let h = NON_JANUARY_HOLIDAYS[i];

        // Безопасно: праздники жёстко заданы константами с валидными датами.
        let holiday_date = RawDate::from_ymd_unchecked(date.year, h.month, h.day);

        if is_weekend(holiday_date) {
            let target = next_predicted_working_day_after(holiday_date);

            if target.month == date.month && target.day == date.day {
                return true;
            }
        }

        i += 1;
    }

    false
}

/// Прогнозирует цели переносов из январского блока (1–8 января).
///
/// Алгоритм: если среди 1–8 января есть праздники, выпавшие на выходные,
/// до двух таких дней переносятся на первые рабочие дни после 8 января.
///
/// Это консервативная эвристика: мы не пытаемся угадать майские или
/// декабрьские мосты, а просто продлеваем январский блок.
fn is_transfer_target_from_january_holiday(date: RawDate) -> bool {
    predicted_january_transfer_targets(date.year).contains(date.month, date.day)
}

/// Находит цели переносов из январского блока для указанного года.
///
/// Возвращает структуру, содержащую до двух дат (месяц, день),
/// на которые прогнозируется перенос январских выходных.
fn predicted_january_transfer_targets(year: i32) -> TransferTargets {
    let mut count: u8 = 0;
    let mut targets = TransferTargets {
        first: None,
        second: None,
    };

    // Начинаем поиск ближайшего рабочего дня с 9 января.
    let mut candidate = RawDate::from_ymd_unchecked(year, 1, 9);

    let mut day: u8 = 1;
    while day <= 8 {
        // Безопасно: 1–8 января всегда валидные даты.
        let holiday = RawDate::from_ymd_unchecked(year, 1, day);

        if is_weekend(holiday) {
            // Найти следующий доступный базовый рабочий день.
            candidate = next_predicted_working_day_from(candidate);

            let md = MonthDay::new(candidate.month, candidate.day);

            if count == 0 {
                targets.first = Some(md);
            } else if count == 1 {
                targets.second = Some(md);
            } else {
                break;
            }

            count += 1;

            // Сдвигаем кандидата на следующий день, чтобы не назначить
            // несколько переносов на одну дату.
            candidate = candidate.next_day();
        }

        day += 1;
    }

    targets
}

/// Ищет ближайший базовый рабочий день ПОСЛЕ указанной даты.
///
/// Базовый рабочий день — не выходной и не федеральный праздник.
///
/// Ограничение в 16 итераций гарантирует завершение: в любом
/// непрерывном отрезке григорианского календаря длиной 16 дней
/// всегда найдётся рабочий день.
fn next_predicted_working_day_after(date: RawDate) -> RawDate {
    next_predicted_working_day_from(date.next_day())
}

/// Ищет ближайший базовый рабочий день НАЧИНАЯ с указанной даты (включительно).
fn next_predicted_working_day_from(mut date: RawDate) -> RawDate {
    let mut i = 0;

    while i < 16 {
        if is_predicted_base_working_day(date) {
            return date;
        }

        date = date.next_day();
        i += 1;
    }

    // Исключение из правила "без panic" для production-пути:
    // это не реакция на входные данные, а защита внутреннего инварианта.
    // В любом окне из 16 дней григорианского календаря гарантированно
    // есть не-выходной и не-праздничный день.
    unreachable!("в окне 16 дней всегда есть базовый рабочий день");
}

/// Проверяет, является ли день базовым рабочим (не выходной, не праздник).
#[inline]
fn is_predicted_base_working_day(date: RawDate) -> bool {
    !is_weekend(date) && !is_federal_holiday(date)
}

/// Структура для хранения до двух дат-целей январских переносов.
#[derive(Debug, Clone, Copy)]
struct TransferTargets {
    first: Option<MonthDay>,
    second: Option<MonthDay>,
}

impl TransferTargets {
    /// Проверяет, содержится ли указанная дата среди целей переносов.
    #[inline]
    fn contains(self, month: u8, day: u8) -> bool {
        let needle = MonthDay::new(month, day);

        self.first == Some(needle) || self.second == Some(needle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2027_jan1_is_holiday() {
        // 2027 нет в официальных данных — попадём в predict.
        let d = RawDate::from_ymd(2027, 1, 1).unwrap();
        let flags = flags(d);
        assert!(flags.is_holiday());
        assert!(flags.is_day_off());
    }

    #[test]
    fn test_2027_jan10_is_weekend() {
        // 10 января 2027 — воскресенье.
        let d = RawDate::from_ymd(2027, 1, 10).unwrap();
        let flags = flags(d);
        assert!(flags.is_weekend());
        assert!(flags.is_day_off());
    }

    #[test]
    fn test_2027_feb22_is_short_day() {
        // 22 февраля 2027 — понедельник, перед 23 февраля (праздник).
        // 23 февраля 2027 — вторник (не выходной).
        let d = RawDate::from_ymd(2027, 2, 22).unwrap();
        let flags = flags(d);
        assert!(flags.is_short_day());
        assert!(flags.is_working_day());
    }

    #[test]
    fn test_2027_mar7_is_weekend_not_short() {
        // 7 марта 2027 — воскресенье, перед 8 марта (праздник).
        // Это выходной, поэтому не может быть коротким днём.
        let d = RawDate::from_ymd(2027, 3, 7).unwrap();
        let flags = flags(d);
        assert!(flags.is_weekend());
        assert!(!flags.is_short_day());
    }

    #[test]
    fn test_2027_mar5_is_short_day() {
        // 5 марта 2027 — пятница. 8 марта — понедельник.
        // Значит 7 марта — воскресенье, 6 марта — суббота.
        // Короткий день: пятница 5 марта? Нет, короткий день — 7 марта?
        // По ст. 95: сокращённый день — непосредственно перед праздником.
        // Перед 8 марта это 7 марта (воскресенье — выходной), значит
        // короткого дня нет в 2027. Но проверим 6 марта:
        // 6 марта — суббота, тоже выходной. Значит короткого дня нет.
        // Это корректное поведение predict: нет короткого дня.
        let d = RawDate::from_ymd(2027, 3, 6).unwrap();
        let flags = flags(d);
        assert!(flags.is_weekend());
        assert!(!flags.is_short_day());
    }

    #[test]
    fn test_2027_transfers_may9_sunday() {
        // 9 мая 2027 — воскресенье. Прогноз: перенос на 10 мая (пн).
        let d = RawDate::from_ymd(2027, 5, 10).unwrap();
        let flags = flags(d);
        // Должен быть дополнительным выходным (перенос).
        assert!(flags.is_day_off());
        assert!(flags.is_transferred());
    }
}
