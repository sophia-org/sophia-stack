---
id: legacy-active-0035
date: 2026-08-27
recorded_date: 2026-08-27
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "tooling"]
---
# 2026-08-27: immutable installation and activation are separate operations

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 1151–1188. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first login intended for the schema-5 packaged-default promotion instead
  archived Hagia attempt `0006` from release `0.1.0-66a279286bdd`, commit
  `66a279286bddd0354b6022102c4dac5254e34481`, and manifest schema 3. Its
  session log recorded no physical or session action before the external
  emergency chord ended it with status 130. The absent Promotion entry and the
  ineffective configured logout chord therefore share one cause: greetd
  launched the old release, not the schema-5 candidate. The emergency run is
  recovery evidence, not promotion evidence.
- The installer treated immutable copying and activation as one operation. If
  a verified new release directory survived but `current`, operator links, or
  greetd entries still named an older release, rerunning installation could
  only refuse to overwrite the existing directory. There was no safe path to
  complete activation.
- `activate_live_session_release.sh` now accepts only the exact real directory
  named by an installed manifest below the configured `releases` root. It
  verifies the complete SHA-256 ledger, packaged policy, Bubblewrap floor,
  required commands, and desktop entries before changing installation state;
  then it repairs command links and desktop files, atomically advances
  `current`, and retains a valid old release as `previous`. Repeating it is
  idempotent. The ordinary argument-free installer selects this path when the
  exact immutable release already exists.
- The isolated installer gate reproduces an old current release with an
  already-installed schema-5 Hagia successor, removes the Promotion entry,
  breaks its command link, and proves both are restored without losing
  rollback. It also rejects activation from the artifact tree. A physical
  greetd refresh and packaged-default run remain the promotion boundary.
- The first privileged retry then stopped before copying: running the packaged
  verifier as root against the build user's artifact made Hagia correctly
  reject its default profile as having an unsafe owner. Installation now checks
  the artifact ledger, copies into a private staging directory, makes that tree
  root-owned for a system install, rechecks its complete ledger, and runs the
  packaged-policy check there before the immutable rename. Non-root isolated
  installs retain their invoking owner. The profile ownership rule remains
  strict; the installer now verifies the ownership the final release will
  actually have.

<!-- END IMPORTED BODY -->
