---
type: "interface"
fqcn: "net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition"
module: "fabric-resource-conditions-api-v1"
sha256: "1d7d9bea7e90eacfb57ac9e6d1f09036c8f118fc5ab50328f55c4d0b8519782c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition

Module: [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] -- kind: interface

```java
public static final com.mojang.serialization.Codec CODEC
public static final com.mojang.serialization.Codec LIST_CODEC
public static final com.mojang.serialization.Codec CONDITION_CODEC
public abstract net.fabricmc.fabric.api.resource.conditions.v1.ResourceConditionType getType()
public abstract boolean test(net.minecraft.resources.RegistryOps$RegistryInfoLookup)
```
