---
type: "interface"
fqcn: "net.minecraft.network.ConnectionProtocol"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.ConnectionProtocol

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `id` | `()Ljava/lang/String;` | exact | invokevirtual@33 in `GlobalReceiverRegistry.logTrackedAddonSize` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `AbstractChanneledNetworkAddon$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `AbstractChanneledNetworkAddon$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@10 in `AbstractChanneledNetworkAddon.getProtocol` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `PayloadTypeRegistryImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `PayloadTypeRegistryImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@9 in `PayloadTypeRegistryImpl.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `VanillaPacketTypes$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `VanillaPacketTypes$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@9 in `VanillaPacketTypes.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/network/ConnectionProtocol;` | exact | invokestatic@0 in `AbstractChanneledNetworkAddon$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/network/ConnectionProtocol;` | exact | invokestatic@0 in `PayloadTypeRegistryImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/network/ConnectionProtocol;` | exact | invokestatic@0 in `VanillaPacketTypes$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CONFIGURATION` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@27 in `AbstractChanneledNetworkAddon$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CONFIGURATION` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@12 in `PayloadTypeRegistryImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CONFIGURATION` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@4 in `PayloadTypeRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CONFIGURATION` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@20 in `PayloadTypeRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CONFIGURATION` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@12 in `VanillaPacketTypes$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CONFIGURATION` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@57 in `ClientConfigurationNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CONFIGURATION` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@24 in `ClientNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CONFIGURATION` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@77 in `ServerConfigurationNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `CONFIGURATION` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@24 in `ServerNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `LOGIN` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@7 in `ClientNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `LOGIN` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@7 in `ServerNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@12 in `AbstractChanneledNetworkAddon$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@74 in `CommonPacketsImpl.lambda$init$1` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@27 in `PayloadTypeRegistryImpl$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@36 in `PayloadTypeRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@52 in `PayloadTypeRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@27 in `VanillaPacketTypes$1.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@68 in `ClientNetworkingImpl.lambda$clientInit$3` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@43 in `ClientNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@46 in `ClientPlayNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@43 in `ServerNetworkingImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `PLAY` | `Lnet/minecraft/network/ConnectionProtocol;` | exact | getstatic@60 in `ServerPlayNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (7 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final HANDSHAKING : Lnet/minecraft/network/ConnectionProtocol;
public static final PLAY : Lnet/minecraft/network/ConnectionProtocol;
public static final STATUS : Lnet/minecraft/network/ConnectionProtocol;
public static final LOGIN : Lnet/minecraft/network/ConnectionProtocol;
public static final CONFIGURATION : Lnet/minecraft/network/ConnectionProtocol;
private final id : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/network/ConnectionProtocol;
public static values()[Lnet/minecraft/network/ConnectionProtocol;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/network/ConnectionProtocol;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
public id()Ljava/lang/String;
private static synthetic $values()[Lnet/minecraft/network/ConnectionProtocol;
static <clinit>()V
```
