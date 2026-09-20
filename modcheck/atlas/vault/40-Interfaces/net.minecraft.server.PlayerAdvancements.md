---
type: "interface"
fqcn: "net.minecraft.server.PlayerAdvancements"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.PlayerAdvancements

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `award` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `setPlayer` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (45, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.PlayerAdvancements {
    private static final org.slf4j.Logger LOGGER;
    private static final com.google.gson.Gson GSON;
    private final net.minecraft.server.players.PlayerList playerList;
    private final java.nio.file.Path playerSavePath;
    private net.minecraft.advancements.AdvancementTree tree;
    private final java.util.Map<net.minecraft.advancements.AdvancementHolder, net.minecraft.advancements.AdvancementProgress> progress;
    private final java.util.Set<net.minecraft.advancements.AdvancementHolder> visible;
    private final java.util.Set<net.minecraft.advancements.AdvancementHolder> progressChanged;
    private final java.util.Set<net.minecraft.advancements.AdvancementNode> rootsToUpdate;
    private net.minecraft.server.level.ServerPlayer player;
    private net.minecraft.advancements.AdvancementHolder lastSelectedTab;
    private boolean isFirstPacket;
    private final com.mojang.serialization.Codec<net.minecraft.server.PlayerAdvancements$Data> codec;
    private final java.util.Map<net.minecraft.advancements.triggers.CriterionTrigger<?>, java.util.Map<net.minecraft.server.PlayerAdvancements$TriggerInstanceKey, ? extends net.minecraft.advancements.CriterionTriggerInstance>> activeTriggers;
    public net.minecraft.server.PlayerAdvancements(com.mojang.datafixers.DataFixer, net.minecraft.server.players.PlayerList, net.minecraft.server.ServerAdvancementManager, java.nio.file.Path, net.minecraft.server.level.ServerPlayer);
    public void setPlayer(net.minecraft.server.level.ServerPlayer);
    public void clearTriggers();
    public void reload(net.minecraft.server.ServerAdvancementManager);
    private void registerListeners(net.minecraft.server.ServerAdvancementManager);
    private void checkForAutomaticTriggers(net.minecraft.server.ServerAdvancementManager);
    private void load(net.minecraft.server.ServerAdvancementManager);
    public void save();
    private void applyFrom(net.minecraft.server.ServerAdvancementManager, net.minecraft.server.PlayerAdvancements$Data);
    private net.minecraft.server.PlayerAdvancements$Data asData();
    public boolean award(net.minecraft.advancements.AdvancementHolder, java.lang.String);
    public boolean revoke(net.minecraft.advancements.AdvancementHolder, java.lang.String);
    private void markForVisibilityUpdate(net.minecraft.advancements.AdvancementHolder);
    private void registerListeners(net.minecraft.advancements.AdvancementHolder);
    private void unregisterListeners(net.minecraft.advancements.AdvancementHolder);
    public void flushDirty(net.minecraft.server.level.ServerPlayer, boolean);
    public void setSelectedTab(net.minecraft.advancements.AdvancementHolder);
    public net.minecraft.advancements.AdvancementProgress getOrStartProgress(net.minecraft.advancements.AdvancementHolder);
    private void startProgress(net.minecraft.advancements.AdvancementHolder, net.minecraft.advancements.AdvancementProgress);
    private void updateTreeVisibility(net.minecraft.advancements.AdvancementNode, java.util.Set<net.minecraft.advancements.AdvancementNode>, java.util.Set<net.minecraft.resources.Identifier>);
    private <T extends net.minecraft.advancements.CriterionTriggerInstance> java.util.Map<net.minecraft.server.PlayerAdvancements$TriggerInstanceKey, T> getOrCreateTriggerMapForType(net.minecraft.advancements.triggers.CriterionTrigger<T>);
    private <T extends net.minecraft.advancements.CriterionTriggerInstance> void addListener(net.minecraft.advancements.triggers.Criterion<T>, net.minecraft.server.PlayerAdvancements$TriggerInstanceKey);
    public <T extends net.minecraft.advancements.CriterionTriggerInstance> java.util.Map<net.minecraft.server.PlayerAdvancements$TriggerInstanceKey, T> getTriggerMapForType(net.minecraft.advancements.triggers.CriterionTrigger<T>);
    private <T extends net.minecraft.advancements.CriterionTriggerInstance> void removeListener(net.minecraft.advancements.triggers.CriterionTrigger<T>, net.minecraft.server.PlayerAdvancements$TriggerInstanceKey);
    private static java.util.Map lambda$getOrCreateTriggerMapForType$0(net.minecraft.advancements.triggers.CriterionTrigger);
    private void lambda$updateTreeVisibility$1(java.util.Set, java.util.Set, net.minecraft.advancements.AdvancementNode, boolean);
    private boolean lambda$updateTreeVisibility$0(net.minecraft.advancements.AdvancementNode);
    private void lambda$award$0(net.minecraft.advancements.AdvancementHolder, net.minecraft.advancements.DisplayInfo);
    private static void lambda$asData$0(java.util.Map, net.minecraft.advancements.AdvancementHolder, net.minecraft.advancements.AdvancementProgress);
    private void lambda$applyFrom$0(net.minecraft.server.ServerAdvancementManager, net.minecraft.resources.Identifier, net.minecraft.advancements.AdvancementProgress);
    static {};
}
```
