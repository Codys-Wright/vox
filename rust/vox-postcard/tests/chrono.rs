use facet::Facet;

#[derive(Facet, Debug, PartialEq)]
struct Dates {
    at: chrono::DateTime<chrono::Utc>,
    day: chrono::NaiveDate,
}

#[test]
fn round_trip_chrono_datetime_and_naive_date() {
    let value = Dates {
        at: chrono::DateTime::parse_from_rfc3339("2026-04-30T15:30:45Z")
            .unwrap()
            .to_utc(),
        day: chrono::NaiveDate::from_ymd_opt(2026, 4, 30).unwrap(),
    };

    let bytes = vox_postcard::to_vec(&value).unwrap();
    let decoded: Dates = vox_postcard::from_slice(&bytes).unwrap();
    assert_eq!(decoded, value);
}
