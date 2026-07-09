//! Example of calendar algebra (`+` and `*`) over `DayFlags`.
//!
//! Demonstrates how to combine federal and regional (Tatarstan)
//! calendars to answer business questions:
//!
//! - `federal + tatarstan`: when is the office closed?
//!   (a day off if at least one calendar marks it as a day off)
//! - `federal * tatarstan`: when do both offices work?
//!   (a working day only if both calendars mark it as working)
//!
//! Run: `cargo run --example calendar_algebra`

use holidays_ru::DayFlags;

fn main() {
    let year = 2026;
    // November: federal holiday (November 4) plus a regional one (November 6, Tatarstan).
    let month = 11;

    println!("=== Calendar algebra: Federal + Tatarstan ===\n");
    println!("November {year} comparison: federal and regional calendars.\n");
    println!(
        "{:-<10} {:-<20} {:-<20} {:-<20} {:-<20}",
        "Date", "Federal", "Tatarstan", "+ (shared day off)", "* (both working)"
    );

    for day in 1..=30 {
        let federal = holidays_ru::flags_ymd::<holidays_ru::Federal>(year, month, day);
        let tatarstan = holidays_ru::flags_ymd::<holidays_ru::regions::Tatarstan>(year, month, day);

        let (fed_flags, tat_flags) = match (federal, tatarstan) {
            (Some(f), Some(t)) => (f.value(), t.value()),
            _ => continue,
        };

        let common_off = fed_flags + tat_flags; // A day off wins.
        let both_work = fed_flags * tat_flags; // A working day wins.

        print!(
            "{year}-{month:02}-{day:02}  {:<20} {:<20} {:<20} {:<20}",
            describe(fed_flags),
            describe(tat_flags),
            describe(common_off),
            describe(both_work),
        );

        // Mark days where the calendars disagree.
        if fed_flags != tat_flags && fed_flags != DayFlags::EMPTY && tat_flags != DayFlags::EMPTY {
            print!(" <- differs");
        }
        println!();
    }

    println!("\n--- How to read this ---");
    println!("  WRK       = working day");
    println!("  WRK+SHT   = shortened working day");
    println!("  WKD+OFF   = weekend day off (Saturday/Sunday)");
    println!("  HOL+OFF   = public holiday day off");
    println!("  -         = day is not marked in this calendar\n");

    println!("--- Rules ---");
    println!("  +  If at least one calendar says 'day off', the result is a day off.");
    println!("     (conservative: treat the day as non-working when in doubt)");
    println!("  *  If at least one calendar says 'working day', the result is a working day.");
    println!("     (permissive: if someone works, work can proceed)\n");

    // Concrete example: November 6, 2026.
    println!("--- Example: November 6, 2026 ---");
    let Some(nov6) = holidays_ru::flags_ymd::<holidays_ru::Federal>(2026, 11, 6) else {
        eprintln!("Invalid federal example date");
        return;
    };
    let Some(nov6_tat) = holidays_ru::flags_ymd::<holidays_ru::regions::Tatarstan>(2026, 11, 6)
    else {
        eprintln!("Invalid Tatarstan example date");
        return;
    };
    let nov6 = nov6.value();
    let nov6_tat = nov6_tat.value();

    println!("  Federal:   {nov6:?}  ->  {}", describe(nov6));
    println!("  Tatarstan: {nov6_tat:?}  ->  {}", describe(nov6_tat));
    println!(
        "  + (shared day off): {:?}  ->  {}",
        nov6 + nov6_tat,
        describe(nov6 + nov6_tat)
    );
    println!(
        "  * (both working):   {:?}  ->  {}",
        nov6 * nov6_tat,
        describe(nov6 * nov6_tat)
    );
    println!();
    println!("  Conclusion: November 6 is a working day for the federal office,");
    println!("  but a day off for the Kazan office (Tatarstan Constitution Day).");
    println!("  + says: the office is closed (a day off wins).");
    println!("  * says: work proceeds (a working day wins, so the Moscow office works).");
}

fn describe(flags: DayFlags) -> String {
    if flags == DayFlags::EMPTY {
        return "-".to_string();
    }

    let mut parts = Vec::new();

    if flags.is_weekend() {
        parts.push("WKD");
    }
    if flags.is_holiday() {
        parts.push("HOL");
    }
    if flags.is_day_off() {
        parts.push("OFF");
    }
    if flags.is_working_day() {
        parts.push("WRK");
    }
    if flags.is_short_day() {
        parts.push("SHT");
    }
    if flags.is_transferred() {
        parts.push("TRF");
    }

    parts.join("+")
}
