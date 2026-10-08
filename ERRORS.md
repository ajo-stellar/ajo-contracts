# Contract errors

The user-facing message column is the canonical wording used by the app.

| Code | Variant | User-facing message |
|---|---|---|
| 1 | CircleNotFound | This circle was not found. Check the circle ID. |
| 10 | Completed | This circle has completed every round. |
| 11 | NotMember | This wallet is not a member of this circle. |
| 12 | AlreadyContributed | This member has already contributed this round. |
| 13 | MissingContributions | Every member must contribute before settlement. |
| 30 | InvalidAmount | Enter a positive contribution amount. |
| 31 | InvalidMembers | A circle requires between 2 and 20 members. |
| 32 | DuplicateMember | Each member address must appear only once. |
| 33 | InvalidRoundLength | Enter a positive round length. |
| 34 | ArithmeticOverflow | The amount or duration is too large. |
| 35 | InvalidAddress | The circle contract cannot be its own token or member. |

Wallet authorization and token failures are host errors, not codes in this table.
