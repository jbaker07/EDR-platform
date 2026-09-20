---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.SimpleReloadInstance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.SimpleReloadInstance

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/packs/resources/ReloadInstance`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `create` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List` | exact | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `create` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List` | exact | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `create` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List` | exact | @ModifyVariable at ['LOAD'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (11 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final PREPARATION_PROGRESS_WEIGHT : I
private static final EXTRA_RELOAD_PROGRESS_WEIGHT : I
private static final LISTENER_PROGRESS_WEIGHT : I
private final allPreparations : Ljava/util/concurrent/CompletableFuture;
private allDone : Ljava/util/concurrent/CompletableFuture;
private final preparingListeners : Ljava/util/Set;
private final listenerCount : I
private final startedTasks : Ljava/util/concurrent/atomic/AtomicInteger;
private final finishedTasks : Ljava/util/concurrent/atomic/AtomicInteger;
private final startedReloads : Ljava/util/concurrent/atomic/AtomicInteger;
private final finishedReloads : Ljava/util/concurrent/atomic/AtomicInteger;
public static of(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;)Lnet/minecraft/server/packs/resources/ReloadInstance;
protected <init>(Ljava/util/List;)V
protected startTasks(Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Lnet/minecraft/server/packs/resources/SimpleReloadInstance$StateFactory;Ljava/util/concurrent/CompletableFuture;)V
protected prepareTasks(Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Lnet/minecraft/server/packs/resources/SimpleReloadInstance$StateFactory;Ljava/util/concurrent/CompletableFuture;)Ljava/util/concurrent/CompletableFuture;
private createBarrierForListener(Lnet/minecraft/server/packs/resources/PreparableReloadListener;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/Executor;)Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;
public done()Ljava/util/concurrent/CompletableFuture;
public getActualProgress()F
private static weightProgress(III)I
public static create(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;
private static synthetic lambda$prepareTasks$4(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Lnet/minecraft/server/packs/resources/PreparableReloadListener;)V
private synthetic lambda$prepareTasks$2(Ljava/util/concurrent/Executor;Ljava/lang/Runnable;)V
private synthetic lambda$prepareTasks$3(Ljava/lang/Runnable;)V
private synthetic lambda$prepareTasks$0(Ljava/util/concurrent/Executor;Ljava/lang/Runnable;)V
private synthetic lambda$prepareTasks$1(Ljava/lang/Runnable;)V
```
