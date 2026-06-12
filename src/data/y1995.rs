use crate::data::{YearFact, months};

/// Официальный производственный календарь на 1995 год.
///
/// По комментарию к календарю дополнительные дни отдыха: 3 и 9 января;
/// предпраздничные дни: 6 января, 7 марта, 8 мая, 6 ноября и 11 декабря.
pub(crate) const Y1995: YearFact = YearFact {
    holidays: months! {
        Jan: [1, 2, 7],
        Mar: [8],
        May: [1, 2, 9],
        Jun: [12],
        Nov: [7],
        Dec: [12],
    },

    extra_days_off: months! {
        Jan: [3, 9],
    },

    working_days: months! {},

    short_days: months! {
        Jan: [6],
        Mar: [7],
        May: [8],
        Nov: [6],
        Dec: [11],
    },

    transferred_days: months! {
        Jan: [1, 3, 7, 9],
    },
};
