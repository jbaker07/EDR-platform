# Captured game diagnostics

Nothing is here yet, and that is the accurate state of this project.

Each file in this directory is verbatim console output from a real Stardew
Valley session with SMAPI and Content Patcher installed, captured by running
the commands a prediction names and pasting the output unedited. One file per
command per case:

    <prediction id>.<command>.txt

for example `sv_deliberate_conflict.summary.txt`.

Alongside them goes a **capture manifest** -- see `capture.template.yaml`. It is
the binding between one prediction and one run: the prediction's id, the hashes
of the files as they sat in the *game's* mod directory, the game, SMAPI and
Content Patcher versions, and each transcript with its own hash.

Judge a prediction with:

    modcheck runtime observe --id <prediction id> \
        --capture evaluation/runtime/transcripts/<id>.<run>.capture.yaml

`observe` validates the binding *before* reading anything, and issues no result
at all if it fails -- not a contradiction and not a pass, because we cannot say
what an unbound transcript is evidence of. It refuses when the manifest names a
different prediction, when the game loaded bytes the prediction is not about,
when a version is unrecorded, or when a transcript file no longer hashes to what
was captured.

`modcheck runtime verify` is a separate, weaker check: it re-hashes files in
*our* cache, which says the prediction still describes the artifacts it was
written for and nothing about what the game loaded. Passing it is not a
substitute for the binding.

This matters most for the probe pack, whose Exclusive and High variants differ
by one field and produce opposite outcomes: without the binding, a transcript
could be filed against the wrong variant and appear to confirm a prediction it
actually contradicts.

## Rules for what may go here

* **Unedited.** Paste what the console printed. Trimming a line that looks like
  noise can remove the reason a patch did not apply.
* **Nothing synthetic.** The transcripts generated in
  `tests/test_observe_contentpatcher.py` exist to test the parser against the
  format Content Patcher's source emits. They are fixtures. Copying one here
  would turn a parser test into a fabricated observation.
* **Record the versions** in the capture manifest. A prediction confirmed on one
  Content Patcher version is not confirmed on all, and `observe` refuses a
  manifest that omits them.
* **Install the complete pack.** The static walkthrough reconstructs a folder
  holding only `content.json`, `manifest.json` and `LICENSE`, which is enough to
  parse and not enough to run: a `Load` whose `FromFile` is missing is skipped
  with a warning, and would look like a resolution outcome. The runtime install
  needs the referenced assets too.
* **Capture the negative deliberately.** A negative claim ("no Load applied")
  is only confirmed from a transcript that demonstrably enumerated the asset --
  an unfiltered `patch summary`, or one filtered to that exact asset. A
  truncated paste, or a summary filtered to a different asset, leaves the claim
  unobserved rather than confirming it.
* **`patch dump order` is not `patch dump applied`.** The first is the global
  definition order and includes patches that never applied. An apply-order claim
  is only answered by the second.
* **Do not treat 'not applied' as a failure without requesting the asset.**
  Content Patcher reports `applied` as false for an asset the game has not
  loaded yet. Summon the horse, enter the farm, open the relevant screen --
  then capture.
* **Config changes need a restart, not a reload.** `patch reload` does not
  refresh ConfigSchema or dynamic tokens, so a capture taken after a reload
  cannot judge `sv_conditional_behaviour`.

## Why this is empty

The analysis container has no game, no SMAPI, and no Stardew Valley licence.
The runtime half of the loop is blocked here, not skipped and not simulated.
The predictions are committed and closed; they can be judged by anyone with a
licensed installation, and until someone does, the observation column stays at
`not_yet_observed`.
