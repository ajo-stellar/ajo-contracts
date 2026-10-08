# Events

Declared with contractevent in src/types.rs.

| Event | Topics | Data |
|---|---|---|
| CircleCreated | circle_created, circle_id | admin, member_count, contribution_amount |
| Contributed | contributed, circle_id, round | member, amount |
| RoundSettled | round_settled, circle_id, round | recipient, amount |
| CircleCompleted | circle_completed, circle_id | received_total, paid_total |

Round indices are zero-based. Amounts are token base units. All public addresses and activity are visible on-chain.
