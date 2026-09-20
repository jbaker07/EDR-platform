# Invoking a coding agent from the creator pipeline

## Where the seam is

`modcheck.creator.agent.CreatorAgent` — one method, `implement(AgentTask) ->
ChangeSet`. The pipeline reaches it in exactly one place: a requirement the
planner marked `grounded` for which no deterministic generator targets the
mechanism. Everywhere a generator exists, the generator runs and the agent is
never called.

What comes back takes the same path as generated code — a `ChangeSet`,
reviewed before anything is written, then the project's own build. Code that
arrives by a different route still has to be reviewable and still has to
compile.

## The blocker, exactly

`AnthropicCreatorAgent.availability()` reports two things, both checkable:

1. **The `anthropic` package is not installed.** It is deliberately not a
   ModCheck dependency: the core does not need it, and adding a dependency the
   core does not need would be wrong. Installing it is an explicit choice.
2. **No credential is resolvable.** `ANTHROPIC_API_KEY` and
   `ANTHROPIC_AUTH_TOKEN` are unset and no `ant auth login` profile exists
   under `~/.config/anthropic`. An unset API key alone would not mean this —
   the SDK resolves all three — so all three are checked.

Supplying those two is the whole remaining step. Nothing in `agent.py` has
been executed against the API from this environment, and it is not claimed to
work.

## What is tested, and what that proves

`tests/test_creator_agent.py` exercises the boundary with a fake agent and no
network: that an agent is reached only where nothing else targets the
mechanism, that the prompt carries the planner's selected mechanism with its
evidence rather than a guess, that returned files become reviewable changes,
that a path escaping the project is refused rather than sanitised, that
unparsable output fails loudly, and that agent output is attributed separately
from generated output.

That establishes the wiring. It establishes nothing about whether a model
given this prompt writes working code, which needs the two missing pieces.

## Provenance, kept separate

`Step.produced_by` is `"generator"` or `"agent"`, set where the code is made
and never inferred afterwards. `Outcome.as_dict()["produced_by"]` splits the
requirements by which produced them, and a test asserts the two sets do not
overlap. Developer-authored work is a third category and does not pass through
this pipeline at all — see `reference/rainlantern/PROVENANCE.md`.

## What was deliberately not built

No model training, no multi-agent platform, no bespoke generator per
demonstration. The agent is one optional argument to `pipeline.run`, and the
default is a `NullCreatorAgent` that refuses and says why — not a stub
returning an empty ChangeSet, which would flow onward and read as "the agent
had nothing to add".
