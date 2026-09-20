---
type: "interface"
fqcn: "net.fabricmc.fabric.api.networking.v1.FriendlyByteBufs"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.FriendlyByteBufs

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: class

```java
public static net.minecraft.network.FriendlyByteBuf empty()
public static net.minecraft.network.FriendlyByteBuf create()
public static net.minecraft.network.FriendlyByteBuf readBytes(io.netty.buffer.ByteBuf, int)
public static net.minecraft.network.FriendlyByteBuf readSlice(io.netty.buffer.ByteBuf, int)
public static net.minecraft.network.FriendlyByteBuf readRetainedSlice(io.netty.buffer.ByteBuf, int)
public static net.minecraft.network.FriendlyByteBuf copy(io.netty.buffer.ByteBuf)
public static net.minecraft.network.FriendlyByteBuf copy(io.netty.buffer.ByteBuf, int, int)
public static net.minecraft.network.FriendlyByteBuf slice(io.netty.buffer.ByteBuf)
public static net.minecraft.network.FriendlyByteBuf retainedSlice(io.netty.buffer.ByteBuf)
public static net.minecraft.network.FriendlyByteBuf slice(io.netty.buffer.ByteBuf, int, int)
public static net.minecraft.network.FriendlyByteBuf retainedSlice(io.netty.buffer.ByteBuf, int, int)
public static net.minecraft.network.FriendlyByteBuf duplicate(io.netty.buffer.ByteBuf)
public static net.minecraft.network.FriendlyByteBuf retainedDuplicate(io.netty.buffer.ByteBuf)
```
