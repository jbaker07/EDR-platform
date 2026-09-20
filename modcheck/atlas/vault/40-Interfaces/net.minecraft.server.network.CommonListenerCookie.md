---
type: "interface"
fqcn: "net.minecraft.server.network.CommonListenerCookie"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.CommonListenerCookie

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createInitial` | `(Lcom/mojang/authlib/GameProfile;Z)Lnet/minecraft/server/network/Commo` | exact | invokestatic@17 in `FakePlayerPacketListener.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (4 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final gameProfile : Lcom/mojang/authlib/GameProfile;
private final latency : I
private final clientInformation : Lnet/minecraft/server/level/ClientInformation;
private final transferred : Z
public <init>(Lcom/mojang/authlib/GameProfile;ILnet/minecraft/server/level/ClientInformation;Z)V
public static createInitial(Lcom/mojang/authlib/GameProfile;Z)Lnet/minecraft/server/network/CommonListenerCookie;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public gameProfile()Lcom/mojang/authlib/GameProfile;
public latency()I
public clientInformation()Lnet/minecraft/server/level/ClientInformation;
public transferred()Z
```
