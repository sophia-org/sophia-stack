---
id: legacy-active-0428
date: 2026-08-15
recorded_date: 2026-08-15
date_basis: heading
imported: 2026-09-06
kind: source
status: historical
tags: ["historical", "rendering", "validation"]
---
# 2026-08-15: mirror attempt 0007 stopped before KMS on an undeclared proof app

Historical source, not a current status claim. <a href="../../../history/research-log-2026-09-06.txt">Original snapshot</a>,
lines 12954–12969. The heading supplies the recorded date.

<!-- BEGIN IMPORTED BODY -->

- The first physical run of signed commit `23880077` produced no native or
  renderer evidence. Argument extensions named application `terminal`, but the
  gate had not declared that application in the normal-session registry, so
  configuration failed immediately with `UnknownApplication("terminal")`.
- The gate now resolves an absolute executable xterm path, explicitly selects
  normal-session mode, declares and starts `terminal`, then applies its bounded
  6x13/white-on-black/scroll arguments. Parser and shell regressions prevent
  terminal arguments from becoming detached from their application declaration
  again.
- Follow-up attempt `0008` also stopped before KMS because the first correction
  combined `--no-config` with the explicit `--desktop-profile`, two intentionally
  exclusive configuration sources. The gate now uses the explicit proof profile
  alone, and its exact parser regression includes that profile argument.

<!-- END IMPORTED BODY -->
