---
type: "interface"
fqcn: "net.minecraft.server.level.ChunkResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ChunkResult

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `ifSuccess` | `(Ljava/util/function/Consumer;)Lnet/minecraft/server/level/ChunkResult` | exact | invokeinterface@7 in `BlockEntityMixin.lambda$fabric_markChanged$0` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (0 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static of(Ljava/lang/Object;)Lnet/minecraft/server/level/ChunkResult;
public static error(Ljava/lang/String;)Lnet/minecraft/server/level/ChunkResult;
public static error(Ljava/util/function/Supplier;)Lnet/minecraft/server/level/ChunkResult;
public abstract isSuccess()Z
public abstract orElse(Ljava/lang/Object;)Ljava/lang/Object;
public static orElse(Lnet/minecraft/server/level/ChunkResult;Ljava/lang/Object;)Ljava/lang/Object;
public abstract getError()Ljava/lang/String;
public abstract ifSuccess(Ljava/util/function/Consumer;)Lnet/minecraft/server/level/ChunkResult;
public abstract map(Ljava/util/function/Function;)Lnet/minecraft/server/level/ChunkResult;
public abstract orElseThrow(Ljava/util/function/Supplier;)Ljava/lang/Object;
private static synthetic lambda$error$0(Ljava/lang/String;)Ljava/lang/String;
```
