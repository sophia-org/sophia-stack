# Architecture Decision Records

[Notebook guide](../README.md) explains creation, acceptance, and supersession.
Current contracts are identified in the [documentation map](../../README.md).

| Record | Status | Scope |
| --- | --- | --- |
| [Separate desktop readiness from application proofs](../decisions/adr0001-separate-desktop-readiness-from-application-proofs.md) | Accepted, recorded retrospectively | Ordinary session lifecycle; physical acceptance remains pending |
| [Session owns desktop composition](../decisions/adr0002-session-owns-desktop-composition.md) | Accepted, recorded retrospectively | Operator component selection and restart semantics |

Use `zk adr --title "The proposed choice"` to start a record. It begins as
`proposed`. Add it here with its status and keep this table consistent when a
decision is accepted, rejected, or superseded. `zk list docs/notes/decisions`
finds records even before they have been added to this map.
