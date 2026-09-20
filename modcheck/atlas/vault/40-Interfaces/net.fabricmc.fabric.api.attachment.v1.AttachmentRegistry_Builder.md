---
type: "interface"
fqcn: "net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry$Builder"
module: "fabric-data-attachment-api-v1"
sha256: "916e1b1046113d5289920fb4058d26ac184e2581f2403b3a482848a2e1dcf7cc"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry$Builder

Module: [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] -- kind: interface

```java
public abstract net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry$Builder persistent(com.mojang.serialization.Codec)
public abstract net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry$Builder copyOnDeath()
public abstract net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry$Builder initializer(java.util.function.Supplier)
public abstract net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry$Builder syncWith(net.minecraft.network.codec.StreamCodec, net.fabricmc.fabric.api.attachment.v1.AttachmentSyncPredicate)
public abstract net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry$Builder syncWith(net.minecraft.network.codec.StreamCodec, net.fabricmc.fabric.api.attachment.v1.AttachmentSyncPredicate, int)
public abstract net.fabricmc.fabric.api.attachment.v1.AttachmentType buildAndRegister(net.minecraft.resources.Identifier)
```
