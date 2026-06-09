/// Битовые маски дней по месяцам.
///
/// Хранит 13 значений `u32` — по одному на месяц (индексы 1–12),
/// где каждый бит соответствует дню месяца (бит 0 не используется).
///
/// Это позволяет проверить наличие дня в категории за O(1).
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MonthMasks([u32; 13]);

impl MonthMasks {
    /// Пустой набор масок (все биты сброшены).
    #[allow(dead_code)]
    pub const EMPTY: Self = Self([0; 13]);

    /// Создаёт `MonthMasks` из массива 13 значений u32.
    #[inline]
    pub const fn new(masks: [u32; 13]) -> Self {
        Self(masks)
    }

    /// Проверяет, установлен ли бит для указанного дня месяца.
    ///
    /// `month` — номер месяца (1–12).
    /// `day` — номер дня (1–31).
    #[inline]
    pub const fn contains(self, month: u8, day: u8) -> bool {
        let mask = self.0[month as usize];
        mask & (1u32 << day) != 0
    }
}

/// Вспомогательная const-функция: упаковывает список дней месяца в битовую маску.
///
/// # Пример
///
/// ```ignore
/// let mask = days([1, 2, 3, 4, 5, 6, 7, 8]);
/// // mask = 0b111111110 = 510
/// ```
#[inline]
pub(crate) const fn days<const N: usize>(days: [u8; N]) -> u32 {
    let mut mask = 0u32;
    let mut i = 0;

    while i < N {
        mask |= 1u32 << days[i];
        i += 1;
    }

    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days() {
        let mask = days([1, 3, 5]);
        assert!(mask & (1 << 1) != 0);
        assert!(mask & (1 << 3) != 0);
        assert!(mask & (1 << 5) != 0);
        assert!(mask & (1 << 2) == 0);
    }

    #[test]
    fn test_month_masks_contains() {
        let masks = MonthMasks::new([0, days([1, 2, 3]), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        assert!(masks.contains(1, 1));
        assert!(masks.contains(1, 3));
        assert!(!masks.contains(1, 4));
        assert!(!masks.contains(2, 1));
    }
}
