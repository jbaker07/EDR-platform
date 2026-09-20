---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.item.ItemVariant"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.item.ItemVariant

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: interface

```java
public static final com.mojang.serialization.Codec CODEC
public static final net.minecraft.network.codec.StreamCodec PACKET_CODEC
public static net.fabricmc.fabric.api.transfer.v1.item.ItemVariant blank()
public static net.fabricmc.fabric.api.transfer.v1.item.ItemVariant of(net.minecraft.world.item.ItemStack)
public static net.fabricmc.fabric.api.transfer.v1.item.ItemVariant of(net.minecraft.world.level.ItemLike)
public static net.fabricmc.fabric.api.transfer.v1.item.ItemVariant of(net.minecraft.world.level.ItemLike, net.minecraft.core.component.DataComponentPatch)
public boolean matches(net.minecraft.world.item.ItemStack)
public net.minecraft.world.item.Item getItem()
public net.minecraft.core.Holder typeHolder()
public net.minecraft.world.item.ItemStack toStack()
public net.minecraft.world.item.ItemStack toStack(int)
public abstract net.fabricmc.fabric.api.transfer.v1.item.ItemVariant withComponents(net.minecraft.core.component.DataComponentPatch)
public net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant withComponents(net.minecraft.core.component.DataComponentPatch)
```
