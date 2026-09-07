---
id: legacy-active-0564
date: 2026-08-29
recorded_date: 2026-08-29
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security", "architecture"]
---
# 2026-08-29: one native protocol family, several authority endpoints

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 17540–17592. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

The developer surface is the protocol, not a Sophia SDK and not the accidental
collection of Rust crates, generators, check scripts, and reference clients
that currently exercise it. A WM or shell author should begin with one family
contract, choose the separately authorized role being implemented, and be able
to complete that role from the published prose and schema using ordinary Unix
IPC in any suitable language.

The family owns the common 24-byte envelope, hello/welcome negotiation,
capability selection, connection epochs, transaction identity, bounded complete
fact sets and candidates, explicit outcomes, recovery, and extension discipline.
Role specifications add only the facts, proposals, and failure semantics
permitted at that authority boundary. Existing wire names may differ -- WM
uses snapshots and projections while the experimental shell uses snapshots and
candidates -- without creating different conceptual lifecycles.

One family deliberately does not mean one socket or one ambient capability.
`sophia_wm_v1`, `sophia_shell_v1`, and `sophia_output_v1` retain distinct
owner-only endpoints, negotiated capabilities, disclosure budgets, protection
domains, schemas, and stability status. The stable WM role cannot acquire shell
metadata by sharing an envelope, and grouping an experimental role into the
family does not promote it.

The source hierarchy is now explicit. Normative architecture owns authority and
disclosure; the family document owns common transport and lifecycle behavior;
the role specification owns role semantics; and the checked-in KDL role schema
owns binary layouts, message kinds, fixed limits, and wire enums. Corpora and
independent clients prove those sources. Generated codecs, headers, bindings,
and tools cannot silently become an alternate specification.

Language neutrality is tested by independence rather than by the number of
official bindings. A stable role must retain a non-Rust client that decodes the
same valid and malformed bytes and completes the minimum full lifecycle without
linking Sophia crates, running a generator, or reading implementation source.
The archived C99 WM client and standalone Nim Hagia client already provide that
evidence for stable `sophia_wm_v1`; the experimental shell owes the equivalent
complete C and Nim proof before stabilization.

The private Rust visual-provider trait is not an exception to the public rule.
It is a trusted renderer implementation seam behind Engine. WM and shell authors
select bounded semantic effects over their language-neutral role IPC and do not
link a provider. Most visual policy must remain expressible through layout,
style, artwork, cached content, display-list primitives, and semantic intents;
provider work is reserved for a genuinely new installed renderer operation.

The roadmap therefore places two coherence gates after the current direct-
scanout critical path and before broad shell vocabulary or shell stabilization:
a role-by-role lifecycle audit and one family-level conformance entry point.
This records the architecture now without interrupting Milestone 14, while
preventing the experimental shell and output paths from hardening into a mixed
bag of disjointed transports and tools.

<!-- END IMPORTED BODY -->
