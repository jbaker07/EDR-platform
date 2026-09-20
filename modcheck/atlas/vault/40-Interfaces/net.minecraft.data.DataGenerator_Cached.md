---
type: "interface"
fqcn: "net.minecraft.data.DataGenerator$Cached"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.DataGenerator$Cached

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

`class` public; extends `net/minecraft/data/DataGenerator`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/nio/file/Path;Lnet/minecraft/WorldVersion;Z)V` | exact | invokespecial@6 in `FabricDataGenerator.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (3 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final rootOutputFolder : Ljava/nio/file/Path;
private final version : Lnet/minecraft/WorldVersion;
private final alwaysGenerate : Z
public <init>(Ljava/nio/file/Path;Lnet/minecraft/WorldVersion;Z)V
public run()V
private synthetic lambda$run$0(Lnet/minecraft/data/HashCache;Lcom/google/common/base/Stopwatch;Ljava/lang/String;Lnet/minecraft/data/DataProvider;)V
```
