# Captured game diagnostics

Nothing is here yet, and that is the accurate state of this project.

Each file in this directory is verbatim console output from a real Stardew
Valley session with SMAPI and Content Patcher installed, captured by running
the commands a prediction names and pasting the output unedited. One file per
command per case:

    <prediction id>.<command>.txt

for example `sv_deliberate_conflict.summary.txt`.

Judge a prediction against them with:

    modcheck runtime verify --id <prediction id>
    modcheck runtime observe --id <prediction id> \
        --summary evaluation/runtime/transcripts/<id>.summary.txt \
        --dump-applied evaluation/runtime/transcripts/<id>.dump-applied.txt

`verify` re-hashes the artifacts the prediction is pinned to and refuses the
pairing if they have changed. That matters most for the probe pack, whose
Exclusive and High variants differ by one field and produce opposite outcomes:
without the hash check, a transcript could be filed against the wrong one and
would appear to confirm a prediction it actually contradicts.

## Rules for what may go here

* **Unedited.** Paste what the console printed. Trimming a line that looks like
  noise can remove the reason a patch did not apply.
* **Nothing synthetic.** The transcripts generated in
  `tests/test_observe_contentpatcher.py` exist to test the parser against the
  format Content Patcher's source emits. They are fixtures. Copying one here
  would turn a parser test into a fabricated observation.
* **Record the versions.** The game, SMAPI and Content Patcher versions belong
  in the first lines of the capture (`patch summary` prints them), because a
  prediction confirmed on one Content Patcher version is not confirmed on all.
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
