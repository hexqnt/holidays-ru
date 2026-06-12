use crate::data::{YearFact, months};

/// Официальный производственный календарь на 1994 год.
///
/// По комментарию к календарю дополнительные дни отдыха: 3-4 января,
/// 3 мая и 13 июня; предпраздничные дни: 6 января и 7 марта.
pub(crate) const Y1994: YearFact = YearFact {
    holidays: months! {
        Jan: [1, 2, 7],
        Mar: [8],
        May: [1, 2, 9],
        Jun: [12],
        Nov: [7],
        Dec: [12],
    },

    extra_days_off: months! {
        Jan: [3, 4],
        May: [3],
        Jun: [13],
    },

    working_days: months! {},

    short_days: months! {
        Jan: [6],
        Mar: [7],
    },

    transferred_days: months! {
        Jan: [1, 2, 3, 4],
        May: [1, 3],
        Jun: [12, 13],
    },
};
