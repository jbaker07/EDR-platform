---
type: "interface"
fqcn: "net.fabricmc.fabric.api.advancement.v1.FabricAdvancementBuilder"
module: "fabric-advancement-api-v1"
sha256: "89e2094ca63a5e3e4687ebd5530eb56a3f051fc568b7ab60243b69a6c8c3e74e"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.advancement.v1.FabricAdvancementBuilder

Module: [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] -- kind: interface

```java
public default java.util.Map<java.lang.String, net.minecraft.advancements.triggers.Criterion<?>> getCriteria()
public default net.minecraft.advancements.Advancement$Builder updateCriteria(java.util.Map<java.lang.String, net.minecraft.advancements.triggers.Criterion<?>>)
public default net.minecraft.advancements.Advancement$Builder removeCriterion(java.lang.String)
public default net.minecraft.advancements.Advancement$Builder requireCriterion(java.lang.String)
public default net.minecraft.advancements.Advancement$Builder requireCriteria(java.util.List<java.lang.String>)
public default net.minecraft.advancements.AdvancementRequirements getRequirements()
public default java.util.Optional<net.minecraft.resources.Identifier> getParent()
public default java.util.Optional<net.minecraft.advancements.DisplayInfo> getDisplay()
public default net.minecraft.advancements.AdvancementRewards getRewards()
public default boolean isSendsTelemetryEvent()
public static net.minecraft.advancements.Advancement$Builder copyOf(net.minecraft.advancements.Advancement)
```
