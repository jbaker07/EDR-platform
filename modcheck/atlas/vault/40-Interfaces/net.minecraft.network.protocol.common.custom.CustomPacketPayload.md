---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.custom.CustomPacketPayload"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.custom.CustomPacketPayload

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `codec` | `(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minecraft/netwo` | exact | invokestatic@10 in `Networking$OpenScreenPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `codec` | `(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minecraft/netwo` | exact | invokestatic@25 in `CommonRegisterPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `codec` | `(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minecraft/netwo` | exact | invokestatic@10 in `CommonVersionPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `codec` | `(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minecraft/netwo` | exact | invokestatic@11 in `RegistrationPayload.codec` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `codec` | `(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minecraft/netwo` | exact | invokestatic@29 in `RegistrySyncPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@14 in `ClientConfigurationNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@14 in `ClientPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@14 in `ServerConfigurationNetworking.createClientboundPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@21 in `ServerConfigurationNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@21 in `ServerPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@14 in `ClientNetworkingImpl.createServerboundPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@14 in `ServerNetworkingImpl.createClientboundPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@8 in `ClientConfigurationNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@8 in `ClientPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@8 in `ServerConfigurationNetworking.createClientboundPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@15 in `ServerConfigurationNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@15 in `ServerPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@1 in `AbstractChanneledNetworkAddon.handle` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@8 in `ClientNetworkingImpl.createServerboundPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@8 in `ServerNetworkingImpl.createClientboundPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@100 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@5 in `ClientboundCustomPayloadPacketMixin.fabric_split` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@40 in `ClientboundCustomPayloadPacketMixin.fabric_split` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@53 in `IdDispatchCodecMixin.encode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@80 in `IdDispatchCodecMixin.encode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@5 in `ServerboundCustomPayloadPacketMixin.fabric_split` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Ty` | exact | invokeinterface@40 in `ServerboundCustomPayloadPacketMixin.fabric_split` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (0 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract type()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Type;
public static codec(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minecraft/network/codec/StreamDecoder;)Lnet/minecraft/network/codec/StreamCodec;
public static createType(Ljava/lang/String;)Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Type;
public static codec(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$FallbackProvider;Ljava/util/List;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$codec$0(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$TypeAndCodec;)Lnet/minecraft/resources/Identifier;
```
