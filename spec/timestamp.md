# Timestamp

## *Type*: `Timestamp`

The number of TAI microseconds since Jan 1 00:00:00.000_000, year 0 in
[ISO 8601:2004](https://en.wikipedia.org/wiki/ISO_8601)

This gives about a range of ±292_470 years since year 0.

This differs from Unix time in 3 ways:

 - Epoch is `0000-01-01T00:00:00.000_000 TAI` instead of
   `1970-01-01T00:00:00.000_000 UTC`
 - Precision is microseconds instead of seconds
 - TAI not UTC, meaning that a leap second gets representation, rather than
   being represented as repeat of previous second as it would be in unix time.

### Fields

 - `micros: long`
