---
type: "interface"
fqcn: "net.minecraft.util.debug.DebugValueSource$Registration"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.debug.DebugValueSource$Registration

System: [[20-Systems/net.minecraft.util.debug|net.minecraft.util.debug]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `register(Lnet/minecraft/util/debug/DebugSubscription;Lnet/minecraft/` | `` | unknown | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |

## Declared members (1, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.util.debug.DebugValueSource$Registration {
    public abstract <T> void register(net.minecraft.util.debug.DebugSubscription<T>, net.minecraft.util.debug.DebugValueSource$ValueGetter<T>);
}
```
