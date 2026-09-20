---
type: "interface"
fqcn: "net.minecraft.server.level.ClientInformation"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ClientInformation

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createDefault` | `()Lnet/minecraft/server/level/ClientInformation;` | exact | invokestatic@7 in `FakePlayer.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (10 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final language : Ljava/lang/String;
private final viewDistance : I
private final chatVisibility : Lnet/minecraft/world/entity/player/ChatVisiblity;
private final chatColors : Z
private final modelCustomisation : I
private final mainHand : Lnet/minecraft/world/entity/HumanoidArm;
private final textFilteringEnabled : Z
private final allowsListing : Z
private final particleStatus : Lnet/minecraft/server/level/ParticleStatus;
public static final MAX_LANGUAGE_LENGTH : I
public <init>(Lnet/minecraft/network/FriendlyByteBuf;)V
public <init>(Ljava/lang/String;ILnet/minecraft/world/entity/player/ChatVisiblity;ZILnet/minecraft/world/entity/HumanoidArm;ZZLnet/minecraft/server/level/ParticleStatus;)V
public write(Lnet/minecraft/network/FriendlyByteBuf;)V
public static createDefault()Lnet/minecraft/server/level/ClientInformation;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public language()Ljava/lang/String;
public viewDistance()I
public chatVisibility()Lnet/minecraft/world/entity/player/ChatVisiblity;
public chatColors()Z
public modelCustomisation()I
public mainHand()Lnet/minecraft/world/entity/HumanoidArm;
public textFilteringEnabled()Z
public allowsListing()Z
public particleStatus()Lnet/minecraft/server/level/ParticleStatus;
```
