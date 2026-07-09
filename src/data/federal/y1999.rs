use crate::data::{YearFact, months};

/// Официальный производственный календарь на 1999 год.
pub(crate) const Y1999: YearFact = YearFact {
    holidays: months! {
        Jan: [1, 2, 7],
        Mar: [8],
        May: [1, 2, 9],
        Jun: [12],
        Nov: [7],
        Dec: [12],
    },

    extra_days_off: months! {
        Jan: [4],
        May: [3, 4, 10],
        Jun: [14],
        Nov: [8],
        Dec: [13],
    },

    working_days: months! {},

    short_days: months! {
        Jan: [6],
        Apr: [30],
        Jun: [11],
        Dec: [31],
    },

    transferred_days: months! {
        Jan: [2, 4],
        May: [1, 2, 3, 4, 9, 10],
        Jun: [12, 14],
        Nov: [7, 8],
        Dec: [12, 13],
    },
};
