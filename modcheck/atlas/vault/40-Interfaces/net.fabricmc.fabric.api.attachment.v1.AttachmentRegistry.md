---
type: "interface"
fqcn: "net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry"
module: "fabric-data-attachment-api-v1"
sha256: "916e1b1046113d5289920fb4058d26ac184e2581f2403b3a482848a2e1dcf7cc"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry

Module: [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] -- kind: class

```java
public static <A> net.fabricmc.fabric.api.attachment.v1.AttachmentType<A> create(net.minecraft.resources.Identifier, java.util.function.Consumer<net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry$Builder<A>>)
public static <A> net.fabricmc.fabric.api.attachment.v1.AttachmentType<A> create(net.minecraft.resources.Identifier)
public static <A> net.fabricmc.fabric.api.attachment.v1.AttachmentType<A> createDefaulted(net.minecraft.resources.Identifier, java.util.function.Supplier<A>)
public static <A> net.fabricmc.fabric.api.attachment.v1.AttachmentType<A> createPersistent(net.minecraft.resources.Identifier, com.mojang.serialization.Codec<A>)
public static <A> net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry$Builder<A> builder()
```
