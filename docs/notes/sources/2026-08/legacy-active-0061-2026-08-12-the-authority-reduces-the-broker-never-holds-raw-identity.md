---
id: legacy-active-0061
date: 2026-08-12
recorded_date: 2026-08-12
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "security"]
---
# 2026-08-12: The authority reduces; the broker never holds raw identity

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1813–1849. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- Four documents disagreed about who sanitizes client metadata, and the
  disagreement decided which process holds every client's title, class, and PID.
  `style-guide.md` said authorities emit sanitized candidates.
  `architecture.md`, `sophia-x-authority.md`, and `compositor-graphics.md` gave
  sanitization to the metadata broker, making raw identity a broker *input*.
- I resolved that inconsistency in the wrong direction first. Seeing three docs
  against one, I edited the style guide to match the majority and recorded it as
  putting the boundary "in one place". Counting documents is not the same as
  reading the design, and the majority was wrong.
- **The code held the better answer.** `XMetadataPropertyCandidate`
  (`sophia-x-authority/src/property.rs:74`) carries the property's name, type, and
  `byte_len` — never its `bytes` — and a test named
  `x11_property_records_emit_metadata_candidates_without_raw_payloads` pins that.
  Someone had already decided raw payloads do not leave the authority, and none of
  the prose said so.
- **Decision: the authority reduces.** The broker publishes a
  `MetadataDisclosureRule` per surface; each authority applies it to text it
  already legitimately holds and emits a bounded label. Raw identity exists in
  exactly one process. The broker keeps what only it can decide — disclosure
  policy, trust assignment, icon tokens, and cross-authority aggregation — because
  those are shared facts and an authority deciding them alone would let two
  authorities disagree about what a user is looking at.
- The argument that lost is worth recording because it is the attractive one:
  centralizing sanitization gives one sanitizer, one policy, one place to audit.
  It also gives one component holding every client's identity across every
  authority, and one more copy of each secret crossing a process boundary to get
  there. Least privilege wins because the thing being centralized — policy — can be
  centralized *without* centralizing the data it governs.
- Distributed reduction costs N implementations of truncate-and-validate. That is
  mechanical, shares one bound and one validator, and is a far smaller drift risk
  than N implementations of policy would have been.
- Lesson worth keeping: when documents disagree, the code may already encode the
  decision, and it is cheaper to look than to take a vote. Four documents now say
  the same thing, and they say it because one test did.

<!-- END IMPORTED BODY -->
