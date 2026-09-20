---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.custom.CustomPacketPayload$1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.custom.CustomPacketPayload$1

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/network/codec/StreamCodec`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `decode` | `(Lnet/minecraft/network/FriendlyByteBuf;)Lnet/minecraft/network/protoc` | exact | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `writeCap` | `(Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/network/protoco` | exact | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (2 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic val$idToType : Ljava/util/Map;
final synthetic val$fallback : Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$FallbackProvider;
 <init>(Ljava/util/Map;Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$FallbackProvider;)V
private findCodec(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/codec/StreamCodec;
private writeCap(Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Type;Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V
public encode(Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V
public decode(Lnet/minecraft/network/FriendlyByteBuf;)Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;
public synthetic encode(Ljava/lang/Object;Ljava/lang/Object;)V
public synthetic decode(Ljava/lang/Object;)Ljava/lang/Object;
```
