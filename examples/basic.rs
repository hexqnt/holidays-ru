//! `holidays-ru` usage example.
//!
//! Run: `cargo run --example basic`

fn main() {
    // YMD API usage (no external dependencies)
    println!("=== holidays-ru usage example ===\n");

    let date = (2026, 1u8, 9u8);
    match holidays_ru::flags_ymd(date.0, date.1, date.2) {
        Some(result) => {
            let flags = result.value();
            println!(
                "{:04}-{:02}-{:02}: {} ({})",
                date.0,
                date.1,
                date.2,
                if flags.is_day_off() {
                    "day off"
                } else {
                    "working day"
                },
                if result.is_fact() { "fact" } else { "predict" },
            );
            println!("  holiday:     {}", flags.is_holiday());
            println!("  short day:   {}", flags.is_short_day());
            println!("  transferred: {}", flags.is_transferred());
        }
        None => println!("{:04}-{:02}-{:02}: invalid date", date.0, date.1, date.2),
    }

    println!();
    println!(
        "Official data years: {}–{}.",
        holidays_ru::FIRST_FACT_YEAR,
        holidays_ru::LAST_FACT_YEAR
    );
    println!(
        "YMD API range: {}–{}; outside official data — algorithmic prediction.",
        holidays_ru::MIN_YEAR,
        holidays_ru::MAX_YEAR
    );
}
