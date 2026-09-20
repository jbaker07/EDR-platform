---
type: "interface"
fqcn: "net.fabricmc.fabric.api.tag.client.v1.ClientTags"
module: "fabric-tag-api-v1"
sha256: "6c2fa7a4d870ee33305c946e5dce6870f92f7f909fa0aa84011d976527e1b3cb"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.tag.client.v1.ClientTags

Module: [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] -- kind: class

```java
public static java.util.Set<net.minecraft.resources.Identifier> getOrCreateLocalTag(net.minecraft.tags.TagKey<?>)
public static <T> boolean isInWithLocalFallback(net.minecraft.tags.TagKey<T>, T)
public static <T> boolean isInWithLocalFallback(net.minecraft.tags.TagKey<T>, net.minecraft.core.Holder<T>)
public static <T> boolean isInLocal(net.minecraft.tags.TagKey<T>, net.minecraft.resources.ResourceKey<T>)
```
