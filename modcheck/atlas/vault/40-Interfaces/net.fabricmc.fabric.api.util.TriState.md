---
type: "interface"
fqcn: "net.fabricmc.fabric.api.util.TriState"
module: "fabric-api-base"
sha256: "88485b1edbcb642fa28b8f53e173835b19f6b6e49aa5b088e3fe16653ef67a13"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.util.TriState

Module: [[30-Mechanisms/fabric-api-base|fabric-api-base]] -- kind: enum

```java
public static final net.fabricmc.fabric.api.util.TriState FALSE
public static final net.fabricmc.fabric.api.util.TriState DEFAULT
public static final net.fabricmc.fabric.api.util.TriState TRUE
public static final com.mojang.serialization.Codec CODEC
public static net.fabricmc.fabric.api.util.TriState[] values()
public static net.fabricmc.fabric.api.util.TriState valueOf(java.lang.String)
public static net.fabricmc.fabric.api.util.TriState of(boolean)
public static net.fabricmc.fabric.api.util.TriState of(java.lang.Boolean)
public boolean get()
public java.lang.Boolean getBoxed()
public boolean orElse(boolean)
public boolean orElseGet(java.util.function.BooleanSupplier)
public java.util.Optional map(net.fabricmc.fabric.api.util.BooleanFunction)
public boolean orElseThrow(java.util.function.Supplier)
public static net.fabricmc.fabric.api.util.TriState fromSystemProperty(java.lang.String)
public java.lang.String getSerializedName()
```
