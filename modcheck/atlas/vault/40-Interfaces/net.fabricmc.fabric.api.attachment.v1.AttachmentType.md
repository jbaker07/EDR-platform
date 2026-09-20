---
type: "interface"
fqcn: "net.fabricmc.fabric.api.attachment.v1.AttachmentType"
module: "fabric-data-attachment-api-v1"
sha256: "916e1b1046113d5289920fb4058d26ac184e2581f2403b3a482848a2e1dcf7cc"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.attachment.v1.AttachmentType

Module: [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] -- kind: interface

```java
public abstract net.minecraft.resources.Identifier identifier()
public abstract com.mojang.serialization.Codec<A> persistenceCodec()
public default boolean isPersistent()
public abstract java.util.function.Supplier<A> initializer()
public abstract boolean isSynced()
public abstract boolean copyOnDeath()
```
