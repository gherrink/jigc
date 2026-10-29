# CI on `fix/rc24-tier1`, 2026-10-04 to 2026-10-27 — the run ids

*Kept 2026-10-29 from the session scratch directory, where the build of the CI fixes left it ([ci-linux-setup-brief.md](ci-linux-setup-brief.md), [ci-flaky-byte-table-brief.md](ci-flaky-byte-table-brief.md)). Two lists read off `gh` on 2026-10-27 — the runs, and each failed run's jobs — joined here into one table; the logs themselves are not kept, because every one is re-fetchable by its id (`gh run view <id> --log-failed`).*

**What it shows.** 95 runs of the `ci` workflow, newest first: the last green run before the stretch is `5f5b273a` (2026-10-04); every one of the 93 runs after it failed `test (g_migrate)` and with it `ci-ok`; the newest, on `495991e3`, was still running when the list was taken. The red was the test's, not the product's — `DECISIONS.md` → *2026-10-27 — CI's one Linux red was the test's, not the product's*. *Failed jobs* is each failed run's jobs whose conclusion was not `success`, as `gh` named them.

| run id | commit | conclusion | date | failed jobs |
|---|---|---|---|---|
| `40544364786` | `495991e3` | running | 2026-10-27 | — |
| `40486130500` | `a7c2d66a` | failure | 2026-10-27 | test (g_migrate):failure,ci-ok:failure |
| `40454874343` | `186bb9e7` | failure | 2026-10-27 | test (g_migrate):failure,ci-ok:failure |
| `40451404489` | `e5e323fc` | failure | 2026-10-27 | test (g_migrate):failure,ci-ok:failure |
| `40448495251` | `e771c46d` | failure | 2026-10-27 | test (g_migrate):failure,ci-ok:failure |
| `39392761072` | `54d28eef` | failure | 2026-10-20 | test (g_migrate):failure,ci-ok:failure |
| `39316537811` | `f505dc2a` | failure | 2026-10-19 | test (g_migrate):failure,ci-ok:failure |
| `39312822775` | `382ebf45` | failure | 2026-10-19 | test (g_migrate):failure,ci-ok:failure |
| `39085427246` | `60094c4f` | failure | 2026-10-18 | test (g_migrate):failure,ci-ok:failure |
| `39022407646` | `462bb0bd` | failure | 2026-10-17 | test (g_migrate):failure,ci-ok:failure |
| `39020384558` | `0a55b4e5` | failure | 2026-10-17 | test (g_migrate):failure,ci-ok:failure |
| `38826838408` | `d54e3011` | failure | 2026-10-15 | test (g_migrate):failure,ci-ok:failure |
| `38813970277` | `fc8f7594` | failure | 2026-10-15 | test (g_migrate):failure,ci-ok:failure |
| `38652700380` | `bc272c1b` | failure | 2026-10-14 | test (g_migrate):failure,ci-ok:failure |
| `38361810411` | `c36d3316` | failure | 2026-10-13 | test (g_migrate):failure,ci-ok:failure |
| `38351118868` | `7725f5d6` | failure | 2026-10-13 | test (g_migrate):failure,ci-ok:failure |
| `38340832775` | `c120cf44` | failure | 2026-10-13 | test (g_migrate):failure,ci-ok:failure |
| `38330969983` | `b7e21223` | failure | 2026-10-13 | test (g_migrate):failure,ci-ok:failure |
| `38322535258` | `bb3eafcf` | failure | 2026-10-12 | test (g_migrate):failure,ci-ok:failure |
| `38307780031` | `dbb1cc84` | failure | 2026-10-12 | test (g_migrate):failure,ci-ok:failure |
| `38299874369` | `a6bb93e5` | failure | 2026-10-12 | test (g_migrate):failure,ci-ok:failure |
| `38297396657` | `b717fb96` | failure | 2026-10-12 | test (g_migrate):failure,ci-ok:failure |
| `38281758029` | `72ca156a` | failure | 2026-10-12 | test (g_migrate):failure,ci-ok:failure |
| `38270386403` | `e619cee8` | failure | 2026-10-12 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38260729762` | `196a4de7` | failure | 2026-10-12 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38251839396` | `bc8fbe85` | failure | 2026-10-12 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38242659827` | `fdd2766e` | failure | 2026-10-12 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38239568967` | `598eb422` | failure | 2026-10-12 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38228028079` | `cb636688` | failure | 2026-10-12 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38218477834` | `adcd6935` | failure | 2026-10-12 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38210002961` | `b7475f3c` | failure | 2026-10-12 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38203988937` | `fbb17209` | failure | 2026-10-12 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38195827669` | `7c376f40` | failure | 2026-10-12 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38194079350` | `19f1674e` | failure | 2026-10-12 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38189559818` | `2ff9d6b9` | failure | 2026-10-12 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38184166846` | `c33ada4a` | failure | 2026-10-11 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38176706888` | `d8d9a738` | failure | 2026-10-11 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38169920130` | `b81cb5b0` | failure | 2026-10-11 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38163521341` | `c313cec6` | failure | 2026-10-11 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38156383270` | `68d14694` | failure | 2026-10-11 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38141203851` | `c4794714` | failure | 2026-10-11 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38135101821` | `8ae40fea` | failure | 2026-10-11 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38130165846` | `b24a6068` | failure | 2026-10-11 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38124267841` | `26564379` | failure | 2026-10-11 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38121464139` | `85661425` | failure | 2026-10-11 | hygiene:failure,test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38108800582` | `a0fa0061` | failure | 2026-10-11 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38104708623` | `9c8b8347` | failure | 2026-10-11 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38100978948` | `3d47c4be` | failure | 2026-10-11 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38096996321` | `b17aa2fe` | failure | 2026-10-11 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38094367160` | `659c5659` | failure | 2026-10-10 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38090623761` | `d591f656` | failure | 2026-10-10 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `38086106030` | `95f77b57` | failure | 2026-10-10 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `38065232644` | `2a5c12f4` | failure | 2026-10-10 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `37713976022` | `7e1b34a1` | failure | 2026-10-08 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `37697718164` | `5dea0cba` | failure | 2026-10-07 | test (g_migrate):failure,ci-ok:failure |
| `37692874207` | `6d30208f` | failure | 2026-10-07 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `37688552987` | `106c5179` | failure | 2026-10-07 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `37678935980` | `083d4745` | failure | 2026-10-07 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `37675225412` | `a182c92e` | failure | 2026-10-07 | test (g_migrate):failure,test (g_tooling):cancelled,ci-ok:failure |
| `37657027920` | `a99b7639` | failure | 2026-10-07 | test (g_migrate):failure,ci-ok:failure |
| `37646372888` | `bbe873eb` | failure | 2026-10-07 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `37636984161` | `047a8ab6` | failure | 2026-10-07 | test (g_tooling):cancelled,test (g_migrate):failure,ci-ok:failure |
| `37634401342` | `c2169e19` | failure | 2026-10-07 | test (g_migrate):failure,ci-ok:failure |
| `37624665960` | `249161b5` | failure | 2026-10-07 | test (g_migrate):failure,test (g_tooling):failure,ci-ok:failure |
| `37615300504` | `b34fb073` | failure | 2026-10-07 | test (g_migrate):failure,test (g_tooling):failure,ci-ok:failure |
| `37601578636` | `aee4224e` | failure | 2026-10-07 | test (g_migrate):failure,test (g_tooling):failure,ci-ok:failure |
| `37598534204` | `e76ef470` | failure | 2026-10-07 | test (g_migrate):failure,test (g_tooling):failure,ci-ok:failure |
| `37588406271` | `1d21024e` | failure | 2026-10-07 | test (g_tooling):failure,test (g_migrate):failure,ci-ok:failure |
| `37582480706` | `b33bf39b` | failure | 2026-10-07 | test (g_migrate):failure,ci-ok:failure |
| `37553425506` | `a7d15955` | failure | 2026-10-07 | test (g_migrate):failure,ci-ok:failure |
| `37544727043` | `cbb3d736` | failure | 2026-10-06 | test (g_migrate):failure,ci-ok:failure |
| `37530319822` | `3129ab91` | failure | 2026-10-06 | test (g_migrate):failure,ci-ok:failure |
| `37522158505` | `473ddd1b` | failure | 2026-10-06 | test (g_tooling):failure,test (g_migrate):failure,ci-ok:failure |
| `37513437924` | `859a99fe` | failure | 2026-10-06 | test (g_migrate):failure,test (g_tooling):failure,ci-ok:failure |
| `37502456561` | `34bf19e4` | failure | 2026-10-06 | test (g_tooling):failure,test (g_migrate):failure,ci-ok:failure |
| `37481495971` | `967e4f58` | failure | 2026-10-06 | test (g_migrate):failure,test (g_tooling):failure,ci-ok:failure |
| `37468345752` | `1cc0be28` | failure | 2026-10-06 | test (g_tooling):failure,test (g_migrate):failure,ci-ok:failure |
| `37458627426` | `463d2691` | failure | 2026-10-06 | test (g_migrate):failure,test (g_tooling):failure,ci-ok:failure |
| `37454033963` | `f3008283` | failure | 2026-10-06 | test (g_tooling):failure,test (g_migrate):failure,ci-ok:failure |
| `37444102164` | `7d732172` | failure | 2026-10-06 | test (g_tooling):failure,test (g_migrate):failure,ci-ok:failure |
| `37442474809` | `755546f1` | failure | 2026-10-06 | test (g_tooling):failure,test (g_migrate):failure,ci-ok:failure |
| `37436058722` | `603018ac` | failure | 2026-10-06 |  |
| `37433420333` | `00825df3` | failure | 2026-10-06 | test (g_migrate):failure,ci-ok:failure |
| `37399373394` | `f6918aaa` | failure | 2026-10-06 | test (g_migrate):failure,ci-ok:failure |
| `37393228813` | `3dbc5612` | failure | 2026-10-06 | test (g_migrate):failure,ci-ok:failure |
| `37387655251` | `5146d080` | failure | 2026-10-05 | test (g_migrate):failure,ci-ok:failure |
| `37378460591` | `f729e52f` | failure | 2026-10-05 | test (g_migrate):failure,ci-ok:failure |
| `37374886301` | `22bcc734` | failure | 2026-10-05 | test (g_migrate):failure,ci-ok:failure |
| `37368965692` | `d3f2445c` | failure | 2026-10-05 | lock-current:cancelled,test (g_flow):cancelled,manifest-freeze:cancelled,test (g_milestone):cancelled,test (g_migration):cancelled,test (g_compose):cancelled,test (g_doc):cancelled,test (g_migrate):failure,ci-ok:failure |
| `37357737538` | `967ca491` | failure | 2026-10-05 | test (g_migrate):failure,ci-ok:failure |
| `37349674744` | `fa2ac313` | failure | 2026-10-05 | test (g_migrate):failure,ci-ok:failure |
| `37296525983` | `357ab425` | failure | 2026-10-05 | test (g_migrate):failure,ci-ok:failure |
| `37293654832` | `822e2edf` | failure | 2026-10-05 | test (g_migrate):failure,ci-ok:failure |
| `37275235922` | `ed783637` | failure | 2026-10-05 | test (g_migrate):failure,ci-ok:failure |
| `37227147846` | `5f5b273a` | success | 2026-10-04 | — |
