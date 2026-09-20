---
type: "interface"
fqcn: "net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(ILnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayloa` | exact | invokespecial@21 in `ClientLoginNetworkAddon.lambda$handlePacket$0` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload` | `()Lnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayloa` | exact | invokevirtual@1 in `ServerLoginNetworkAddon.handle` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload` | `()Lnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayloa` | exact | invokevirtual@19 in `ServerLoginPacketListenerImplMixin.handleCustomPayloadReceivedAsync` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `transactionId` | `()I` | exact | invokevirtual@10 in `ServerLoginNetworkAddon.handle` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `readPayload` | `(ILnet/minecraft/network/FriendlyByteBuf;)Lnet/minecraft/network/proto` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `MAX_PAYLOAD_SIZE` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |

## Declared members (4 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final transactionId : I
private final payload : Lnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayload;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private static final MAX_PAYLOAD_SIZE : I
public <init>(ILnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayload;)V
private static read(Lnet/minecraft/network/FriendlyByteBuf;)Lnet/minecraft/network/protocol/login/ServerboundCustomQueryAnswerPacket;
private static readPayload(ILnet/minecraft/network/FriendlyByteBuf;)Lnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayload;
private static readUnknownPayload(Lnet/minecraft/network/FriendlyByteBuf;)Lnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayload;
private write(Lnet/minecraft/network/FriendlyByteBuf;)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/login/ServerLoginPacketListener;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public transactionId()I
public payload()Lnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayload;
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
private static synthetic lambda$write$0(Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayload;)V
static <clinit>()V
```
