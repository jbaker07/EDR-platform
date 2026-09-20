---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.WorldCreationUiState$WorldTypeEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.WorldCreationUiState$WorldTypeEntry

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/core/Holder;)V` | exact | invokespecial@30 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (2 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final preset : Lnet/minecraft/core/Holder;
private static final CUSTOM_WORLD_DESCRIPTION : Lnet/minecraft/network/chat/Component;
public <init>(Lnet/minecraft/core/Holder;)V
public describePreset()Lnet/minecraft/network/chat/Component;
public isAmplified()Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public preset()Lnet/minecraft/core/Holder;
private static synthetic lambda$isAmplified$0(Lnet/minecraft/resources/ResourceKey;)Z
private static synthetic lambda$describePreset$0(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/network/chat/Component;
static <clinit>()V
```
