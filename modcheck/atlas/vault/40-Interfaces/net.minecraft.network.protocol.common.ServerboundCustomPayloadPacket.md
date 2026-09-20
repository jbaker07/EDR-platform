---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/network/protocol/Packet`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V` | exact | invokespecial@36 in `ClientNetworkingImpl.createServerboundPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;` | exact | invokevirtual@18 in `IdDispatchCodecMixin.encode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;` | exact | invokevirtual@1 in `ServerCommonPacketListenerImplMixin.handleCustomPayloadReceivedAsync` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;` | exact | invokevirtual@5 in `ServerGamePacketListenerImplMixin.handleCustomPayloadReceivedAsync` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `payload` | `Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| wraps | `<clinit>` | `()V` | exact | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (3 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final payload : Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;
private static final MAX_PAYLOAD_SIZE : I
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/common/ServerCommonPacketListener;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public payload()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
private static synthetic lambda$static$1(Ljava/util/ArrayList;)V
private static synthetic lambda$static$0(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/codec/StreamCodec;
static <clinit>()V
```
