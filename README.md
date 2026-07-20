# cron-union

Merge multiple cron expressions into one deduped set of compiled schedules.

Supports 5-field, 6-field, and 7-field cron expressions, plus `@daily`-style aliases.

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

### Alias support

```text
A = @daily
B = 0 0 0 * * *
A ∪ B = [@daily]
```

## Validation

Enable the pre-push hook once:

```text
git config core.hooksPath .githooks
```
