# Date

## *Type*: `Date`

A naïve date (unspecified timezone)

### Fields

 - `year: half` Range: 0 ~ 65\_535
 - `month: byte` Range: 1 ~ 12
 - `day: byte` Range: (of week: 1 ~ 7) << 5 | (of month: 1 ~ 31)
