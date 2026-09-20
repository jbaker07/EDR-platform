---
type: "interface"
fqcn: "net.fabricmc.fabric.api.resource.conditions.v1.ResourceConditions"
module: "fabric-resource-conditions-api-v1"
sha256: "1d7d9bea7e90eacfb57ac9e6d1f09036c8f118fc5ab50328f55c4d0b8519782c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.resource.conditions.v1.ResourceConditions

Module: [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] -- kind: class

```java
public static final java.lang.String CONDITIONS_KEY
public static final java.lang.String OVERLAYS_KEY
public static void register(net.fabricmc.fabric.api.resource.conditions.v1.ResourceConditionType)
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceConditionType getConditionType(net.minecraft.resources.Identifier)
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition alwaysTrue()
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition alwaysFalse()
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition not(net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition)
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition and(net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition[])
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition or(net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition[])
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition allModsLoaded(java.lang.String[])
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition anyModsLoaded(java.lang.String[])
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition tagsPopulated(net.minecraft.tags.TagKey[])
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition tagsPopulated(net.minecraft.resources.ResourceKey, net.minecraft.tags.TagKey[])
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition featuresEnabled(net.minecraft.resources.Identifier[])
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition featuresEnabled(net.minecraft.world.flag.FeatureFlag[])
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition registryContains(net.minecraft.resources.ResourceKey[])
public static net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition registryContains(net.minecraft.resources.ResourceKey, net.minecraft.resources.Identifier[])
```
