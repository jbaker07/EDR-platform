---
type: "question"
id: "q.bedrock_artifacts"
kind: "missing_artifact_access"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.bedrock_artifacts

**Question.** What does the Bedrock Edition modification surface consist of at the artifact level (behaviour pack and resource pack manifests, the @minecraft/server scripting modules and their type declarations), and can they be retrieved under terms that allow extraction?

**Kind.** `missing_artifact_access` -- **Status.** open

**Why it matters.** Bedrock is the larger player base and a completely separate surface. Without artifacts the atlas can only say it exists.

**Affects.** [[10-Workflows/wf.content.data_driven_content|wf.content.data_driven_content]], [[10-Workflows/wf.behaviour.entity_ai|wf.behaviour.entity_ai]]

**Evidence already available.**
- `extracted/corpus.json`

**Best remaining source.** The npm packages @minecraft/server and @minecraft/server-ui (type declarations), and Mojang's bedrock-samples repository, if their terms permit.

**Procedure.** Retrieve with `modcheck sources add`, record reuse terms, add a `bedrock` group to atlas/extract/corpus.py, and write a TypeScript-declaration extractor parallel to jvm.py.

**Done when.** corpus.json has a bedrock group with hashed artifacts and at least one extracted surface note.

**Conclusions affected while open.**
- Nothing in this atlas applies to Bedrock; any request naming Bedrock is out of corpus.
