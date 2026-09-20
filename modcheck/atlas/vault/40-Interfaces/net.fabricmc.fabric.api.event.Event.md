---
type: "interface"
fqcn: "net.fabricmc.fabric.api.event.Event"
module: "fabric-api-base"
sha256: "88485b1edbcb642fa28b8f53e173835b19f6b6e49aa5b088e3fe16653ef67a13"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.Event

Module: [[30-Mechanisms/fabric-api-base|fabric-api-base]] -- kind: abstract_class

```java
protected volatile T invoker
public static final net.minecraft.resources.Identifier DEFAULT_PHASE
public net.fabricmc.fabric.api.event.Event()
public final T invoker()
public abstract void register(T)
public void register(net.minecraft.resources.Identifier, T)
public void addPhaseOrdering(net.minecraft.resources.Identifier, net.minecraft.resources.Identifier)
static {}
```
