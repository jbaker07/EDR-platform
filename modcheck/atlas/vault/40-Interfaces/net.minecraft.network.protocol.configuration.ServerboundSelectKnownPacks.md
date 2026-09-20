---
type: "interface"
fqcn: "net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<clinit>` | `()V` | exact | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (2 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final knownPacks : Ljava/util/List;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Ljava/util/List;)V
public type()Lnet/minecraft/network/protocol/PacketType;
public handle(Lnet/minecraft/network/protocol/configuration/ServerConfigurationPacketListener;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public knownPacks()Ljava/util/List;
public synthetic handle(Lnet/minecraft/network/PacketListener;)V
static <clinit>()V
```
