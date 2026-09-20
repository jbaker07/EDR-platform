---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.MultiPlayerGameMode"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.MultiPlayerGameMode

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `startPrediction` | `(Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/client/m` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| calls | `stopDestroyBlock` | `()V` | exact | invokevirtual@18 in `MinecraftMixin.injectHandleBlockBreakingForCancelling` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `attack` | `(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `continueDestroyBlock` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `destroyBlock` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `startDestroyBlock` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `useItem` | `(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/Intera` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `useItemOn` | `(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/world/Interac` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| reads | `destroyBlockPos` | `Lnet/minecraft/core/BlockPos;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| reads | `destroyingItem` | `Lnet/minecraft/world/item/ItemStack;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| wraps | `sameDestroyTarget` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (14 fields, 50 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DEFAULT_DESTROY_COOLDOWN_TICKS : I
private final minecraft : Lnet/minecraft/client/Minecraft;
private final connection : Lnet/minecraft/client/multiplayer/ClientPacketListener;
private destroyBlockPos : Lnet/minecraft/core/BlockPos;
private destroyingItem : Lnet/minecraft/world/item/ItemStack;
private destroyProgress : F
private destroyTicks : F
private destroyDelay : I
private destroyDirection : Lnet/minecraft/core/Direction;
private isDestroying : Z
private localPlayerMode : Lnet/minecraft/world/level/GameType;
private previousLocalPlayerMode : Lnet/minecraft/world/level/GameType;
private carriedIndex : I
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/multiplayer/ClientPacketListener;)V
public adjustPlayer(Lnet/minecraft/world/entity/player/Player;)V
public setLocalMode(Lnet/minecraft/world/level/GameType;Lnet/minecraft/world/level/GameType;)V
public setLocalMode(Lnet/minecraft/world/level/GameType;)V
public canHurtPlayer()Z
public destroyBlock(Lnet/minecraft/core/BlockPos;)Z
public startDestroyBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z
public stopDestroyBlock()V
public continueDestroyBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z
private startPrediction(Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/client/multiplayer/prediction/PredictiveAction;)V
public tick()V
private sameDestroyTarget(Lnet/minecraft/core/BlockPos;)Z
private ensureHasSentCarriedItem()V
public useItemOn(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;
private performUseItemOn(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;
public useItem(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
public createPlayer(Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/stats/StatsCounter;Lnet/minecraft/client/ClientRecipeBook;Lnet/minecraft/client/player/ItemActivation;)Lnet/minecraft/client/player/LocalPlayer;
public createPlayer(Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/stats/StatsCounter;Lnet/minecraft/client/ClientRecipeBook;Lnet/minecraft/world/entity/player/Input;ZLnet/minecraft/client/player/ItemActivation;)Lnet/minecraft/client/player/LocalPlayer;
public attack(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Entity;)V
public spectate(Lnet/minecraft/world/entity/Entity;)V
public spectatorNoAction()V
public interact(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/EntityHitResult;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
public handleContainerInput(IIILnet/minecraft/world/inventory/ContainerInput;Lnet/minecraft/world/entity/player/Player;)V
public handlePlaceRecipe(ILnet/minecraft/world/item/crafting/display/RecipeDisplayId;Z)V
public handleInventoryButtonClick(II)V
public handleCreativeModeItemAdd(Lnet/minecraft/world/item/ItemStack;I)V
public handleCreativeModeItemDrop(Lnet/minecraft/world/item/ItemStack;)V
public releaseUsingItem(Lnet/minecraft/world/entity/player/Player;)V
public piercingAttack(Lnet/minecraft/world/item/component/SwingAnimation;Lnet/minecraft/world/item/component/PiercingWeapon;)V
public hasExperience()Z
public hasMissTime()Z
public isServerControlledInventory()Z
public isSpectator()Z
public getPreviousPlayerMode()Lnet/minecraft/world/level/GameType;
public getPlayerMode()Lnet/minecraft/world/level/GameType;
public isDestroying()Z
public getDestroyStage()I
public handlePickItemFromBlock(Lnet/minecraft/core/BlockPos;Z)V
public handlePickItemFromEntity(Lnet/minecraft/world/entity/Entity;Z)V
public handleSlotStateChanged(IIZ)V
public dropItem(Lnet/minecraft/client/player/LocalPlayer;Z)V
private synthetic lambda$useItem$0(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/entity/player/Player;Lorg/apache/commons/lang3/mutable/MutableObject;I)Lnet/minecraft/network/protocol/Packet;
private static synthetic lambda$useItem$1(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$performUseItemOn$0(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemStack;
private synthetic lambda$useItemOn$0(Lorg/apache/commons/lang3/mutable/MutableObject;Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockHitResult;I)Lnet/minecraft/network/protocol/Packet;
private synthetic lambda$continueDestroyBlock$1(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;I)Lnet/minecraft/network/protocol/Packet;
private synthetic lambda$continueDestroyBlock$0(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;I)Lnet/minecraft/network/protocol/Packet;
private synthetic lambda$startDestroyBlock$1(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;I)Lnet/minecraft/network/protocol/Packet;
private synthetic lambda$startDestroyBlock$0(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;I)Lnet/minecraft/network/protocol/Packet;
static <clinit>()V
```
