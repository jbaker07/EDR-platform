---
type: "interface"
fqcn: "net.minecraft.world.level.GameType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.GameType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`enum` public final; extends `java/lang/Enum`; implements `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `SURVIVAL` | `Lnet/minecraft/world/level/GameType;` | exact | getstatic@8 in `ServerPlayerMixin.fakePlayerGameMode` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (15 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SURVIVAL : Lnet/minecraft/world/level/GameType;
public static final CREATIVE : Lnet/minecraft/world/level/GameType;
public static final ADVENTURE : Lnet/minecraft/world/level/GameType;
public static final SPECTATOR : Lnet/minecraft/world/level/GameType;
public static final DEFAULT_MODE : Lnet/minecraft/world/level/GameType;
public static final CODEC : Lnet/minecraft/util/StringRepresentable$EnumCodec;
private static final BY_ID : Ljava/util/function/IntFunction;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final OPTIONAL_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final LEGACY_ID_CODEC : Lcom/mojang/serialization/Codec;
private final id : I
private final name : Ljava/lang/String;
private final shortName : Lnet/minecraft/network/chat/Component;
private final longName : Lnet/minecraft/network/chat/Component;
private static final synthetic $VALUES : [Lnet/minecraft/world/level/GameType;
public static values()[Lnet/minecraft/world/level/GameType;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/level/GameType;
private <init>(Ljava/lang/String;IILjava/lang/String;)V
public getId()I
public getName()Ljava/lang/String;
public getSerializedName()Ljava/lang/String;
public getLongDisplayName()Lnet/minecraft/network/chat/Component;
public getShortDisplayName()Lnet/minecraft/network/chat/Component;
public updatePlayerAbilities(Lnet/minecraft/world/entity/player/Abilities;)V
public isBlockPlacingRestricted()Z
public isCreative()Z
public isSurvival()Z
public static byId(I)Lnet/minecraft/world/level/GameType;
public static byName(Ljava/lang/String;)Lnet/minecraft/world/level/GameType;
public static byName(Ljava/lang/String;Lnet/minecraft/world/level/GameType;)Lnet/minecraft/world/level/GameType;
public static isValidId(I)Z
private static synthetic $values()[Lnet/minecraft/world/level/GameType;
private static synthetic lambda$isValidId$0(ILnet/minecraft/world/level/GameType;)Z
private static synthetic lambda$static$1(Ljava/util/Optional;)Ljava/util/OptionalInt;
private static synthetic lambda$static$0(Ljava/util/OptionalInt;)Ljava/util/Optional;
static <clinit>()V
```
