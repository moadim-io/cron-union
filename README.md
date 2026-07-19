# cron-union

Merge multiple cron expressions into one deduped schedule.

## Math

If each cron expression is a set of fire times:

```text
S = A ∪ B ∪ C
```

If two expressions fire at the same timestamp, that timestamp appears once.

## Example

```text
A = {1, 5, 10}
B = {5, 7}
A ∪ B = {1, 5, 7, 10}
```
