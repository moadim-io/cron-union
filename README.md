# cron-union

Merge multiple cron expressions into one deduped set of compiled schedules.

## Math

If each cron expression is a set of fire times:

```text
S = A ∪ B ∪ C
```

If two expressions fire at the same timestamp, that timestamp appears once.

## Examples

### Merge two schedules

```text
A = 0 * * * *
B = */30 * * * *

Result = union(A, B) = [*/30 * * * *]
```

### Dedupe redundant crons

```text
A = 0 * * * *
B = */30 * * * *
A ∪ B = [*/30 * * * *]
```
