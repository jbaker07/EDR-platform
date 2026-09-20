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
public default <A> A getAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>)
public default <A> A getAttachedOrThrow(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>)
public default <A> A getAttachedOrSet(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>, A)
public default <A> A getAttachedOrCreate(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>, java.util.function.Supplier<A>)
public default <A> A getAttachedOrCreate(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>)
public default <A> A getAttachedOrElse(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>, A)
public default <A> A getAttachedOrGet(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>, java.util.function.Supplier<A>)
public default <A> A setAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>, A)
public default boolean hasAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType<?>)
public default <A> A removeAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>)
public default <A> net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.attachment.v1.AttachmentTarget$OnAttachedSet<A>> onAttachedSet(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>)
public default <A> A modifyAttached(net.fabricmc.fabric.api.attachment.v1.AttachmentType<A>, java.util.function.UnaryOperator<A>)
```
