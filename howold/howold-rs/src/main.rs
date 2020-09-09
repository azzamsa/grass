use chrono::prelude::*;

fn main() {
    let today = Local::today();
    let birth_date = Local.ymd(2000, 11, 28);

    let age = today - birth_date;
    let years = age.num_days() / 365;

    println!("You were born at: {}", birth_date.format("%A, %d %B %Y"));
    // FIXME sadly chrono rust doesn't have num_years and num_month
    // https://github.com/chronotope/chrono/issues/416
    println!("You age: {} years", years);
}
