---
type: "interface"
fqcn: "net.fabricmc.fabric.api.attachment.v1.AttachmentTarget"
module: "fabric-data-attachment-api-v1"
sha256: "916e1b1046113d5289920fb4058d26ac184e2581f2403b3a482848a2e1dcf7cc"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.attachment.v1.AttachmentTarget

Module: [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] -- kind: interface

```java
public static final java.lang.String NBT_ATTACHMENT_KEY
public java.lang.Object getAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType)
public java.lang.Object getAttachedOrThrow(net.fabricmc.fabric.api.attachment.v1.AttachmentType)
public java.lang.Object getAttachedOrSet(net.fabricmc.fabric.api.attachment.v1.AttachmentType, java.lang.Object)
public java.lang.Object getAttachedOrCreate(net.fabricmc.fabric.api.attachment.v1.AttachmentType, java.util.function.Supplier)
public java.lang.Object getAttachedOrCreate(net.fabricmc.fabric.api.attachment.v1.AttachmentType)
public java.lang.Object getAttachedOrElse(net.fabricmc.fabric.api.attachment.v1.AttachmentType, java.lang.Object)
public java.lang.Object getAttachedOrGet(net.fabricmc.fabric.api.attachment.v1.AttachmentType, java.util.function.Supplier)
public java.lang.Object setAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType, java.lang.Object)
public boolean hasAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType)
public java.lang.Object removeAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType)
public net.fabricmc.fabric.api.event.Event onAttachedSet(net.fabricmc.fabric.api.attachment.v1.AttachmentType)
public java.lang.Object modifyAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType, java.util.function.UnaryOperator)
```
