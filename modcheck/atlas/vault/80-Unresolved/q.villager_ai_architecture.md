---
type: "question"
id: "q.villager_ai_architecture"
kind: "unmodeled_behaviour"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.villager_ai_architecture

**Question.** Villagers in 26.3 use a Brain (LivingEntity.getBrain() is declared on a hooked type) rather than the GoalSelector that Mob exposes; how are villager behaviours registered, and is there any Fabric API hook into brain construction?

**Kind.** `unmodeled_behaviour` -- **Status.** open

**Why it matters.** The two AI systems need different modification strategies. Mob.goalSelector is a protected field (extracted); Brain behaviours are not on any hooked type.

**Affects.** [[70-Requests/request.villager_fear|request.villager_fear]], [[10-Workflows/wf.behaviour.entity_ai|wf.behaviour.entity_ai]]

**Evidence already available.**
- [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]]
- [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]]
- `extracted/minecraft_members.json`

**Best remaining source.** net.minecraft.world.entity.npc and net.minecraft.world.entity.ai.behavior packages, extracted as extra types.

**Procedure.** Extract Villager, VillagerGoalPackages/behaviour registration classes and Brain; record which method builds the activity map; note the absence or presence of a Fabric event.

**Done when.** The villager_fear request lists a verified injection target or a verified API.

**Conclusions affected while open.**
- Candidate designs for villager behaviour name classes whose 26.3 signatures are unverified.
