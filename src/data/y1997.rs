use crate::data::{YearFact, months};

/// Официальный производственный календарь на 1997 год.
pub(crate) const Y1997: YearFact = YearFact {
    holidays: months! {
        Jan: [1, 2, 7],
        Mar: [8],
        May: [1, 2, 9],
        Jun: [12],
        Nov: [7],
        Dec: [12],
    },

    extra_days_off: months! {
        Mar: [10],
    },

    working_days: months! {},

    short_days: months! {
        Jan: [6],
        Mar: [7],
        Apr: [30],
        May: [8],
        Jun: [11],
        Nov: [6],
        Dec: [11, 31],
    },

    transferred_days: months! {
        Mar: [8, 10],
    },
};
