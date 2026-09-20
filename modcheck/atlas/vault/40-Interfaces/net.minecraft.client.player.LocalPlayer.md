---
type: "interface"
fqcn: "net.minecraft.client.player.LocalPlayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.player.LocalPlayer

System: [[20-Systems/net.minecraft.client.player|net.minecraft.client.player]]

`class` public; extends `net/minecraft/client/player/AbstractClientPlayer`; implements `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getAbilities` | `()Lnet/minecraft/world/entity/player/Abilities;` | inherited_exact | invokevirtual@7 in `MultiPlayerGameModeMixin.method_2902` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getMainHandItem` | `()Lnet/minecraft/world/item/ItemStack;` | inherited_exact | invokevirtual@1 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getMainHandItem` | `()Lnet/minecraft/world/item/ItemStack;` | inherited_exact | invokevirtual@26 in `MultiPlayerGameModeMixin.fabricItemContinueBlockBreakingInject` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getOffhandItem` | `()Lnet/minecraft/world/item/ItemStack;` | inherited_exact | invokevirtual@47 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `getRotationVector` | `()Lnet/minecraft/world/phys/Vec2;` | inherited_exact | invokevirtual@6 in `FabricClientCommandSource.getRotation` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `isShiftKeyDown` | `()Z` | exact | invokevirtual@95 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `isSpectator` | `()Z` | inherited_exact | invokevirtual@1 in `MultiPlayerGameModeMixin.interactBlock` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@17 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@19 in `MultiPlayerGameModeMixin.interactBlock` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@51 in `MultiPlayerGameModeMixin.interactBlock` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@7 in `LevelExtractorMixin.getParticleMaterialProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `lookAt` | `(Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;Lnet/mi` | inherited_exact | invokevirtual@29 in `TestInputImpl.lambda$lookAt$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `position` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@6 in `FabricClientCommandSource.getPosition` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `setXRot` | `(F)V` | inherited_exact | invokevirtual@31 in `TestInputImpl.lambda$lookAt$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setYRot` | `(F)V` | inherited_exact | invokevirtual@23 in `TestInputImpl.lambda$lookAt$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `swing` | `(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/compon` | inherited_exact | invokevirtual@137 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `aiStep` | `()V` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `isSprintingPossible` | `(Z)Z` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `shouldStopSwimSprinting` | `()Z` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| reads | `connection` | `Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | getfield@96 in `RecipeSyncImplClient.onRecipeSyncPacket` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (55 fields, 113 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final LOGGER : Lorg/slf4j/Logger;
private static final POSITION_REMINDER_INTERVAL : I
private static final WATER_VISION_MAX_TIME : I
private static final WATER_VISION_QUICK_TIME : I
private static final WATER_VISION_QUICK_PERCENT : F
private static final SUFFOCATING_COLLISION_CHECK_SCALE : D
private static final MINOR_COLLISION_ANGLE_THRESHOLD_RADIAN : D
private static final PORTAL_SPINNING_SPEED : F
private static final NAUSEA_SPINNING_SPEED : F
public final connection : Lnet/minecraft/client/multiplayer/ClientPacketListener;
private final stats : Lnet/minecraft/stats/StatsCounter;
private final recipeBook : Lnet/minecraft/client/ClientRecipeBook;
private final dropSpamThrottler : Lnet/minecraft/util/TickThrottler;
private final ambientSoundHandlers : Ljava/util/List;
private permissions : Lnet/minecraft/server/permissions/PermissionSet;
private chatAbilities : Lnet/minecraft/client/multiplayer/chat/ChatAbilities;
private xLast : D
private yLast : D
private zLast : D
private yRotLast : F
private xRotLast : F
private lastOnGround : Z
private lastHorizontalCollision : Z
private crouching : Z
private wasSprinting : Z
private positionReminder : I
private flashOnSetHealth : Z
public input : Lnet/minecraft/client/player/ClientInput;
private lastSentInput : Lnet/minecraft/world/entity/player/Input;
protected final minecraft : Lnet/minecraft/client/Minecraft;
protected sprintTriggerTime : I
private static final EXPERIENCE_DISPLAY_UNREADY_TO_SET : I
private static final EXPERIENCE_DISPLAY_READY_TO_SET : I
public experienceDisplayStartTick : I
public yBob : F
public xBob : F
public yBobO : F
public xBobO : F
private jumpRidingTicks : I
private jumpRidingScale : F
public portalEffectIntensity : F
public oPortalEffectIntensity : F
private spinningEffectTime : F
private spinningEffectSpeed : F
private final itemActivation : Lnet/minecraft/client/player/ItemActivation;
private final firstPersonHandsAndItems : Lnet/minecraft/client/player/FirstPersonHandsAndItems;
private startedUsingItem : Z
private usingItemHand : Lnet/minecraft/world/InteractionHand;
private handsBusy : Z
private autoJumpEnabled : Z
private autoJumpTime : I
private wasFallFlying : Z
private waterVisionTime : I
private showDeathScreen : Z
private doLimitedCrafting : Z
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/client/multiplayer/ClientPacketListener;Lnet/minecraft/stats/StatsCounter;Lnet/minecraft/client/ClientRecipeBook;Lnet/minecraft/world/entity/player/Input;ZLnet/minecraft/client/multiplayer/chat/ChatAbilities;Lnet/minecraft/client/player/ItemActivation;)V
public heal(F)V
public startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z
public removeVehicle()V
public getViewYRot(F)F
public tick()V
public displayItemActivation(Lnet/minecraft/world/item/ItemStack;)V
public resetItemActivation()V
public itemActivation()Lnet/minecraft/client/player/ItemActivation;
public itemUsed(Lnet/minecraft/world/InteractionHand;)V
public firstPersonHandsAndItems()Lnet/minecraft/client/player/FirstPersonHandsAndItems;
public sendChanges()V
public getCurrentMood()F
private sendPosition()V
private sendIsSprintingIfNeeded()V
public respawn()V
public closeContainer()V
public clientSideCloseContainer()V
public hurtTo(F)V
public onUpdateAbilities()V
public setReducedDebugInfo(Z)V
public isLocalPlayer()Z
public isSuppressingSlidingDownLadder()Z
public canSpawnSprintParticle()Z
protected sendRidingJump()V
public sendOpenInventory()V
public getStats()Lnet/minecraft/stats/StatsCounter;
public getRecipeBook()Lnet/minecraft/client/ClientRecipeBook;
public removeRecipeHighlight(Lnet/minecraft/world/item/crafting/display/RecipeDisplayId;)V
public permissions()Lnet/minecraft/server/permissions/PermissionSet;
public setPermissions(Lnet/minecraft/server/permissions/PermissionSet;)V
public chatAbilities()Lnet/minecraft/client/multiplayer/chat/ChatAbilities;
public refreshChatAbilities()V
public sendSystemMessage(Lnet/minecraft/network/chat/Component;)V
public sendOverlayMessage(Lnet/minecraft/network/chat/Component;)V
private moveTowardsClosestSpace(DD)V
private suffocatesAt(Lnet/minecraft/core/BlockPos;)Z
public setExperienceValues(FII)V
private setExperienceDisplayStartTickToTickCount()V
public handleEntityEvent(B)V
public setShowDeathScreen(Z)V
public shouldShowDeathScreen()Z
public setDoLimitedCrafting(Z)V
public getDoLimitedCrafting()Z
public playSound(Lnet/minecraft/sounds/SoundEvent;FF)V
public startUsingItem(Lnet/minecraft/world/InteractionHand;)V
public isUsingItem()Z
private isSlowDueToUsingItem()Z
private itemUseSpeedMultiplier()F
public stopUsingItem()V
public getUsedItemHand()Lnet/minecraft/world/InteractionHand;
public onSyncedDataUpdated(Lnet/minecraft/network/syncher/EntityDataAccessor;)V
public jumpableVehicle()Lnet/minecraft/world/entity/PlayerRideableJumping;
public getJumpRidingScale()F
public isTextFilteringEnabled()Z
public openTextEdit(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraft/world/level/block/entity/SignTextSlot;)V
public openMinecartCommandBlock(Lnet/minecraft/world/entity/vehicle/minecart/MinecartCommandBlock;)V
public openCommandBlock(Lnet/minecraft/world/level/block/entity/CommandBlockEntity;)V
public openStructureBlock(Lnet/minecraft/world/level/block/entity/StructureBlockEntity;)V
public openTestBlock(Lnet/minecraft/world/level/block/entity/TestBlockEntity;)V
public openTestInstanceBlock(Lnet/minecraft/world/level/block/entity/TestInstanceBlockEntity;)V
public openJigsawBlock(Lnet/minecraft/world/level/block/entity/JigsawBlockEntity;)V
public openDialog(Lnet/minecraft/core/Holder;)V
public openItemGui(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/InteractionHand;)V
public crit(Lnet/minecraft/world/entity/Entity;)V
public magicCrit(Lnet/minecraft/world/entity/Entity;)V
public isShiftKeyDown()Z
public isCrouching()Z
public isMovingSlowly()Z
public applyInput()V
private modifyInput(Lnet/minecraft/world/phys/Vec2;)Lnet/minecraft/world/phys/Vec2;
private static modifyInputSpeedForSquareMovement(Lnet/minecraft/world/phys/Vec2;)Lnet/minecraft/world/phys/Vec2;
private static distanceToUnitSquare(Lnet/minecraft/world/phys/Vec2;)F
protected isControlledCamera()Z
public resetPos()V
public aiStep()V
private shouldStopRunSprinting()Z
private shouldStopSwimSprinting()Z
public getActivePortalLocalTransition()Lnet/minecraft/world/level/block/Portal$Transition;
protected tickDeath()V
private handlePortalTransitionEffect(Z)V
private tickSpinningEffect()V
public getSpinningEffectAngle(F)F
public rideTick()V
public isHandsBusy()Z
public move(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V
public isAutoJumpEnabled()Z
public shouldRotateWithMinecart()Z
protected updateAutoJump(FF)V
protected isHorizontalCollisionMinor(Lnet/minecraft/world/phys/Vec3;)Z
private canAutoJump()Z
private isMoving()Z
private isSprintingPossible(Z)Z
private canStartSprinting()Z
private vehicleCanSprint(Lnet/minecraft/world/entity/Entity;)Z
public getWaterVision()F
public onGameModeChanged(Lnet/minecraft/world/level/GameType;)V
public isUnderWater()Z
protected updateIsUnderwater()Z
public getRopeHoldPosition(F)Lnet/minecraft/world/phys/Vec3;
public updateTutorialInventoryAction(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/inventory/ClickAction;)V
public getVisualRotationYInDegrees()F
public handleCreativeModeItemDrop(Lnet/minecraft/world/item/ItemStack;)V
public canDropItems()Z
public getDropSpamThrottler()Lnet/minecraft/util/TickThrottler;
public getLastSentInput()Lnet/minecraft/world/entity/player/Input;
public raycastHitResult(FLnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/phys/HitResult;
private static pick(Lnet/minecraft/world/entity/Entity;DDF)Lnet/minecraft/world/phys/HitResult;
private static filterHitResult(Lnet/minecraft/world/phys/HitResult;Lnet/minecraft/world/phys/Vec3;D)Lnet/minecraft/world/phys/HitResult;
public setActivePostEffects(Ljava/util/List;)V
public getActivePostEffects()Ljava/util/List;
private static synthetic lambda$updateAutoJump$0(Lnet/minecraft/world/phys/shapes/VoxelShape;)Ljava/util/stream/Stream;
static <clinit>()V
```
