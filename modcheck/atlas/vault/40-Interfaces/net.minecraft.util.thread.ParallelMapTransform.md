---
type: "interface"
fqcn: "net.minecraft.util.thread.ParallelMapTransform"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.thread.ParallelMapTransform

System: [[20-Systems/net.minecraft.util.thread|net.minecraft.util.thread]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `schedule(Ljava/util/Map;Ljava/util/function/BiFunction;Ljava/util/co` | `` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.thread.ParallelMapTransform {
    private static final int DEFAULT_TASKS_PER_THREAD;
    public net.minecraft.util.thread.ParallelMapTransform();
    public static <K, U, V> java.util.concurrent.CompletableFuture<java.util.Map<K, V>> schedule(java.util.Map<K, U>, java.util.function.BiFunction<K, U, V>, int, java.util.concurrent.Executor);
    public static <K, U, V> java.util.concurrent.CompletableFuture<java.util.Map<K, V>> schedule(java.util.Map<K, U>, java.util.function.BiFunction<K, U, V>, java.util.concurrent.Executor);
    private static java.util.Map lambda$schedule$0(java.util.function.BiFunction, java.lang.Object, java.lang.Object);
}
```
