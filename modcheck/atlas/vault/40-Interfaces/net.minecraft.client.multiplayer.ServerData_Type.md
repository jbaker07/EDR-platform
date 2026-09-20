---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ServerData$Type"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ServerData$Type

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `OTHER` | `Lnet/minecraft/client/multiplayer/ServerData$Type;` | exact | getstatic@10 in `TestDedicatedServerContextImpl.lambda$connect$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (4 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final LAN : Lnet/minecraft/client/multiplayer/ServerData$Type;
public static final REALM : Lnet/minecraft/client/multiplayer/ServerData$Type;
public static final OTHER : Lnet/minecraft/client/multiplayer/ServerData$Type;
private static final synthetic $VALUES : [Lnet/minecraft/client/multiplayer/ServerData$Type;
public static values()[Lnet/minecraft/client/multiplayer/ServerData$Type;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/client/multiplayer/ServerData$Type;
private <init>(Ljava/lang/String;I)V
private static synthetic $values()[Lnet/minecraft/client/multiplayer/ServerData$Type;
static <clinit>()V
```
