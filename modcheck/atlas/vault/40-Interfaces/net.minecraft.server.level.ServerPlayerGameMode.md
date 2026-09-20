---
type: "interface"
fqcn: "net.minecraft.server.level.ServerPlayerGameMode"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerPlayerGameMode

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `destroyBlock` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `destroyBlock` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `handleBlockBreakAction` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/network/protocol/game/Ser` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `useItem` | `(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/L` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `useItemOn` | `(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/L` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `level` | `Lnet/minecraft/server/level/ServerLevel;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| reads | `player` | `Lnet/minecraft/server/level/ServerPlayer;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |

## Declared members (15 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final FLIGHT_DISABLE_RANGE : D
private static final LOGGER : Lorg/slf4j/Logger;
protected level : Lnet/minecraft/server/level/ServerLevel;
protected final player : Lnet/minecraft/server/level/ServerPlayer;
private gameModeForPlayer : Lnet/minecraft/world/level/GameType;
private previousGameModeForPlayer : Lnet/minecraft/world/level/GameType;
private isDestroyingBlock : Z
private destroyProgressStart : I
private destroyPos : Lnet/minecraft/core/BlockPos;
private destroyDirection : Lnet/minecraft/core/Direction;
private gameTicks : I
private hasDelayedDestroy : Z
private delayedDestroyPos : Lnet/minecraft/core/BlockPos;
private delayedTickStart : I
private lastSentState : I
public <init>(Lnet/minecraft/server/level/ServerPlayer;)V
public changeGameModeForPlayer(Lnet/minecraft/world/level/GameType;)Z
protected setGameModeForPlayer(Lnet/minecraft/world/level/GameType;Lnet/minecraft/world/level/GameType;)V
private isInRangeOfGround()Z
public getGameModeForPlayer()Lnet/minecraft/world/level/GameType;
public getPreviousGameModeForPlayer()Lnet/minecraft/world/level/GameType;
public isSurvival()Z
public isCreative()Z
public tick()V
private incrementDestroyProgress(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;I)F
private debugLogging(Lnet/minecraft/core/BlockPos;ZILjava/lang/String;)V
public handleBlockBreakAction(Lnet/minecraft/core/BlockPos;Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket$Action;Lnet/minecraft/core/Direction;II)V
private abortDestroyBlock(Lnet/minecraft/core/BlockPos;I)V
public destroyAndAck(Lnet/minecraft/core/BlockPos;ILjava/lang/String;)V
public destroyBlock(Lnet/minecraft/core/BlockPos;)Z
public useItem(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
public useItemOn(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;
public setLevel(Lnet/minecraft/server/level/ServerLevel;)V
private static synthetic lambda$useItemOn$0(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemStack;
private synthetic lambda$handleBlockBreakAction$0(Lnet/minecraft/world/item/ItemStack;)V
static <clinit>()V
```
