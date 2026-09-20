---
type: "interface"
fqcn: "net.minecraft.world.InteractionResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.InteractionResult

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `consumesAction` | `()Z` | exact | invokeinterface@42 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `consumesAction` | `()Z` | exact | invokeinterface@65 in `MultiPlayerGameModeMixin.fabric_fireAttackBlockCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `consumesAction` | `()Z` | exact | invokeinterface@41 in `MultiPlayerGameModeMixin.interactBlock` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `FAIL` | `Lnet/minecraft/world/InteractionResult$Fail;` | exact | getstatic@36 in `InteractionEventsRouter.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `FAIL` | `Lnet/minecraft/world/InteractionResult$Fail;` | exact | getstatic@75 in `InteractionEventsRouter.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@43 in `AttackBlockCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@58 in `AttackBlockCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@43 in `AttackEntityCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@58 in `AttackEntityCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@41 in `UseBlockCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@56 in `UseBlockCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@43 in `UseEntityCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@58 in `UseEntityCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@39 in `UseItemCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@54 in `UseItemCallback.lambda$static$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@79 in `InteractionEventsRouter.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@45 in `PlayerMixin.onPlayerInteractEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@69 in `ServerGamePacketListenerImplMixin.handleInteract` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@39 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@24 in `ServerPlayerGameModeMixin.interactBlock` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@22 in `ServerPlayerGameModeMixin.interactItem` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@34 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@37 in `MultiPlayerGameModeMixin.fabric_fireAttackBlockCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@33 in `MultiPlayerGameModeMixin.interactBlock` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@24 in `MultiPlayerGameModeMixin.interactItem` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASS` | `Lnet/minecraft/world/InteractionResult$Pass;` | exact | getstatic@28 in `MultiPlayerGameModeMixin.attackEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `SUCCESS` | `Lnet/minecraft/world/InteractionResult$Success;` | exact | getstatic@46 in `MultiPlayerGameModeMixin.fabric_fireAttackBlockCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `SUCCESS` | `Lnet/minecraft/world/InteractionResult$Success;` | exact | getstatic@32 in `MultiPlayerGameModeMixin.interactItem` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `SUCCESS` | `Lnet/minecraft/world/InteractionResult$Success;` | exact | getstatic@36 in `MultiPlayerGameModeMixin.attackEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (6 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SUCCESS : Lnet/minecraft/world/InteractionResult$Success;
public static final SUCCESS_SERVER : Lnet/minecraft/world/InteractionResult$Success;
public static final CONSUME : Lnet/minecraft/world/InteractionResult$Success;
public static final FAIL : Lnet/minecraft/world/InteractionResult$Fail;
public static final PASS : Lnet/minecraft/world/InteractionResult$Pass;
public static final TRY_WITH_EMPTY_HAND : Lnet/minecraft/world/InteractionResult$TryEmptyHandInteraction;
public consumesAction()Z
static <clinit>()V
```
