---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ServerData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ServerData

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/client/multiplayer` | exact | invokespecial@13 in `TestDedicatedServerContextImpl.lambda$connect$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (16 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final MAX_ICON_SIZE : I
public name : Ljava/lang/String;
public ip : Ljava/lang/String;
public status : Lnet/minecraft/network/chat/Component;
public motd : Lnet/minecraft/network/chat/Component;
public players : Lnet/minecraft/network/protocol/status/ServerStatus$Players;
public ping : J
public protocol : I
public version : Lnet/minecraft/network/chat/Component;
public playerList : Ljava/util/List;
private packStatus : Lnet/minecraft/client/multiplayer/ServerData$ServerPackStatus;
private iconBytes : [B
private type : Lnet/minecraft/client/multiplayer/ServerData$Type;
private acceptedCodeOfConduct : I
private state : Lnet/minecraft/client/multiplayer/ServerData$State;
public <init>(Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/client/multiplayer/ServerData$Type;)V
public write()Lnet/minecraft/nbt/CompoundTag;
public getResourcePackStatus()Lnet/minecraft/client/multiplayer/ServerData$ServerPackStatus;
public setResourcePackStatus(Lnet/minecraft/client/multiplayer/ServerData$ServerPackStatus;)V
public static read(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/client/multiplayer/ServerData;
public getIconBytes()[B
public setIconBytes([B)V
public isLan()Z
public isRealm()Z
public type()Lnet/minecraft/client/multiplayer/ServerData$Type;
public hasAcceptedCodeOfConduct(Ljava/lang/String;)Z
public acceptCodeOfConduct(Ljava/lang/String;)V
public clearCodeOfConduct()V
public copyNameIconFrom(Lnet/minecraft/client/multiplayer/ServerData;)V
public copyFrom(Lnet/minecraft/client/multiplayer/ServerData;)V
public state()Lnet/minecraft/client/multiplayer/ServerData$State;
public setState(Lnet/minecraft/client/multiplayer/ServerData$State;)V
public static validateIcon([B)[B
static <clinit>()V
```
