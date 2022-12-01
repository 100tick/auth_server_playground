use chrono::{Datelike, NaiveDate, Utc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn rand_string(len: usize) -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    rand_string
}

pub fn random_date() -> NaiveDate {
    let mut rng = thread_rng();

    let y = rng.gen_range(1990..Utc::now().year());
    let m = rng.gen_range(1..12);
    let d = rng.gen_range(1..28);

    NaiveDate::from_ymd(y, m, d)
}

#[test]
fn test_rand_string() {
    let string1 = rand_string(10);
    assert_eq!(10, string1.len());

    let string2 = rand_string(5);
    assert_eq!(5, string2.len());

    println!("{}", string1);
    println!("{}", string2);
}
