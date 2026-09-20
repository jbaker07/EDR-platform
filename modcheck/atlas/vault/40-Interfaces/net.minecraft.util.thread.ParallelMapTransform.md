---
type: "interface"
fqcn: "net.minecraft.util.thread.ParallelMapTransform"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.thread.ParallelMapTransform

System: [[20-Systems/net.minecraft.util.thread|net.minecraft.util.thread]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `schedule` | `(Ljava/util/Map;Ljava/util/function/BiFunction;Ljava/util/concurrent/E` | exact | invokestatic@23 in `ModelBakeryMixin.withExtraModels` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (1 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DEFAULT_TASKS_PER_THREAD : I
public <init>()V
public static schedule(Ljava/util/Map;Ljava/util/function/BiFunction;ILjava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
public static schedule(Ljava/util/Map;Ljava/util/function/BiFunction;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$schedule$0(Ljava/util/function/BiFunction;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
```
