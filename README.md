# cron-union

Merge multiple cron expressions into one deduped schedule.

## Math

If each cron expression is a set of fire times:

```text
S = A ∪ B ∪ C
```

If two expressions fire at the same timestamp, that timestamp appears once.

## Examples

### Merge two schedules

```text
A = */5 * * * *
B = 0 * * * *

Result = union(A, B)
```

### Dedupe overlaps

```text
A = {1, 5, 10}
B = {5, 7}
A ∪ B = {1, 5, 7, 10}
```

### Same timestamp, one run

```text
A fires at 10:00
B fires at 10:00

Output: 10:00 once
```
