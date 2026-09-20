---
type: "interface"
fqcn: "net.fabricmc.fabric.api.event.EventFactory"
module: "fabric-api-base"
sha256: "88485b1edbcb642fa28b8f53e173835b19f6b6e49aa5b088e3fe16653ef67a13"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.EventFactory

Module: [[30-Mechanisms/fabric-api-base|fabric-api-base]] -- kind: class

```java
public static <T> net.fabricmc.fabric.api.event.Event<T> createArrayBacked(java.lang.Class<? super T>, java.util.function.Function<T[], T>)
public static <T> net.fabricmc.fabric.api.event.Event<T> createArrayBacked(java.lang.Class<T>, T, java.util.function.Function<T[], T>)
public static <T> net.fabricmc.fabric.api.event.Event<T> createWithPhases(java.lang.Class<? super T>, java.util.function.Function<T[], T>, net.minecraft.resources.Identifier...)
public static java.lang.String getHandlerName(java.lang.Object)
public static boolean isProfilingEnabled()
public static void invalidate()
```
