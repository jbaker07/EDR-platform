---
type: "interface"
fqcn: "net.minecraft.advancements.Advancement$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.Advancement$Builder

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `build(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/advance` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| injects_into | `addCriterion(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/advancements/Advancement$Builder;` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.advancements.Advancement$Builder {
    private java.util.Optional<net.minecraft.resources.Identifier> parent;
    private java.util.Optional<net.minecraft.advancements.DisplayInfo> display;
    private net.minecraft.advancements.AdvancementRewards rewards;
    private final com.google.common.collect.ImmutableMap$Builder<java.lang.String, net.minecraft.advancements.triggers.Criterion<?>> criteria;
    private java.util.Optional<net.minecraft.advancements.AdvancementRequirements> requirements;
    private net.minecraft.advancements.AdvancementRequirements$Strategy requirementsStrategy;
    private boolean sendsTelemetryEvent;
    public net.minecraft.advancements.Advancement$Builder();
    public static net.minecraft.advancements.Advancement$Builder advancement();
    public static net.minecraft.advancements.Advancement$Builder recipeAdvancement();
    public net.minecraft.advancements.Advancement$Builder parent(net.minecraft.advancements.AdvancementHolder);
    public net.minecraft.advancements.Advancement$Builder parent(net.minecraft.resources.Identifier);
    public net.minecraft.advancements.Advancement$Builder rootDisplay(net.minecraft.world.item.ItemStackTemplate, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component, net.minecraft.resources.Identifier, net.minecraft.advancements.AdvancementType, boolean, boolean, boolean);
    public net.minecraft.advancements.Advancement$Builder rootDisplay(net.minecraft.world.item.Item, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component, net.minecraft.resources.Identifier, net.minecraft.advancements.AdvancementType, boolean, boolean, boolean);
    public net.minecraft.advancements.Advancement$Builder display(net.minecraft.world.item.ItemStackTemplate, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component, net.minecraft.advancements.AdvancementType, boolean, boolean, boolean);
    public net.minecraft.advancements.Advancement$Builder display(net.minecraft.world.item.Item, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component, net.minecraft.advancements.AdvancementType, boolean, boolean, boolean);
    public net.minecraft.advancements.Advancement$Builder display(net.minecraft.advancements.DisplayInfo);
    public net.minecraft.advancements.Advancement$Builder rewards(net.minecraft.advancements.AdvancementRewards$Builder);
    public net.minecraft.advancements.Advancement$Builder rewards(net.minecraft.advancements.AdvancementRewards);
    public net.minecraft.advancements.Advancement$Builder addCriterion(java.lang.String, net.minecraft.advancements.triggers.Criterion<?>);
    public net.minecraft.advancements.Advancement$Builder requirements(net.minecraft.advancements.AdvancementRequirements$Strategy);
    public net.minecraft.advancements.Advancement$Builder requirements(net.minecraft.advancements.AdvancementRequirements);
    public net.minecraft.advancements.Advancement$Builder sendsTelemetryEvent();
    public net.minecraft.advancements.AdvancementHolder build(net.minecraft.resources.Identifier);
    public net.minecraft.advancements.AdvancementHolder save(net.minecraft.data.worldgen.BootstrapContext<net.minecraft.advancements.Advancement>, java.lang.String);
    private net.minecraft.advancements.AdvancementRequirements lambda$build$0(java.util.Map);
}
```
