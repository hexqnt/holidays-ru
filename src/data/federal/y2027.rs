use crate::data::{YearFact, months};

/// Официальный производственный календарь на 2027 год.
///
/// На основе Постановления Правительства РФ от 17.09.2026 № 1187
/// «О переносе выходных дней в 2027 году».
pub(crate) const Y2027: YearFact = YearFact {
    holidays: months! {
        Jan: [1, 2, 3, 4, 5, 6, 7, 8],
        Feb: [23],
        Mar: [8],
        May: [1, 9],
        Jun: [12],
        Nov: [4],
    },
    extra_days_off: months! {
        Feb: [22],
        May: [3, 10],
        Jun: [14],
        Nov: [5],
        Dec: [31],
    },
    working_days: months! {
        Feb: [20],
    },
    short_days: months! {
        Feb: [20],
        Apr: [30],
        Jun: [11],
        Nov: [3],
    },
    // Источники и цели переносов, включая переносы по ст. 112 ТК РФ.
    transferred_days: months! {
        Jan: [2, 3],
        Feb: [20, 22],
        May: [1, 3, 9, 10],
        Jun: [12, 14],
        Nov: [5],
        Dec: [31],
    },
};
