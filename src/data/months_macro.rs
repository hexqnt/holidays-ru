/// Макрос для читаемого определения `MonthMasks`.
///
/// Принимает пары `Месяц: [дни]` с трёхбуквенными английскими названиями
/// месяцев (`Jan`, `Feb`, ..., `Dec`) и создаёт `MonthMasks`
/// с установленными битами для указанных дней. Неуказанные месяцы
/// получают нулевую маску.
///
/// # Пример использования (внутренний)
///
/// ```ignore
/// const HOLIDAYS: MonthMasks = months! {
///     Jan: [1, 2, 3, 4, 5, 6, 7, 8],
///     May: [1, 9],
/// };
/// ```
///
/// Этот макрос разворачивается в const-выражение — ноль накладных расходов в runtime.
macro_rules! months {
    // Входная точка: пары Месяц: [дни]
    ($($month:ident: [$($day:literal),* $(,)?]),* $(,)?) => {
        const {
            #[allow(unused_mut)]
            let mut masks = [0u32; 13];
            $(
                masks[months!(@idx $month)] = $crate::data::days([$($day),*]);
            )*
            $crate::data::MonthMasks::new(masks)
        }
    };

    // Вспомогательные правила: имя месяца → номер (1–12)
    (@idx Jan) => { 1 };
    (@idx Feb) => { 2 };
    (@idx Mar) => { 3 };
    (@idx Apr) => { 4 };
    (@idx May) => { 5 };
    (@idx Jun) => { 6 };
    (@idx Jul) => { 7 };
    (@idx Aug) => { 8 };
    (@idx Sep) => { 9 };
    (@idx Oct) => { 10 };
    (@idx Nov) => { 11 };
    (@idx Dec) => { 12 };
}

pub(crate) use months;
