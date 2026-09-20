---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.SimpleReloadInstance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.SimpleReloadInstance

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `create(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | `@ModifyArg at INVOKE Lnet/minecraft/server/packs/resources/SimpleReloadInstance;` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `create(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | `@ModifyArg at INVOKE Lnet/minecraft/server/packs/resources/ProfiledReloadInstanc` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `create(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | `@ModifyVariable at LOAD` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.packs.resources.SimpleReloadInstance<S> implements net.minecraft.server.packs.resources.ReloadInstance {
    private static final int PREPARATION_PROGRESS_WEIGHT;
    private static final int EXTRA_RELOAD_PROGRESS_WEIGHT;
    private static final int LISTENER_PROGRESS_WEIGHT;
    private final java.util.concurrent.CompletableFuture<net.minecraft.util.Unit> allPreparations;
    private java.util.concurrent.CompletableFuture<java.util.List<S>> allDone;
    private final java.util.Set<net.minecraft.server.packs.resources.PreparableReloadListener> preparingListeners;
    private final int listenerCount;
    private final java.util.concurrent.atomic.AtomicInteger startedTasks;
    private final java.util.concurrent.atomic.AtomicInteger finishedTasks;
    private final java.util.concurrent.atomic.AtomicInteger startedReloads;
    private final java.util.concurrent.atomic.AtomicInteger finishedReloads;
    public static net.minecraft.server.packs.resources.ReloadInstance of(net.minecraft.server.packs.resources.ResourceManager, java.util.List<net.minecraft.server.packs.resources.PreparableReloadListener>, java.util.concurrent.Executor, java.util.concurrent.Executor, java.util.concurrent.CompletableFuture<net.minecraft.util.Unit>);
    protected net.minecraft.server.packs.resources.SimpleReloadInstance(java.util.List<net.minecraft.server.packs.resources.PreparableReloadListener>);
    protected void startTasks(java.util.concurrent.Executor, java.util.concurrent.Executor, net.minecraft.server.packs.resources.ResourceManager, java.util.List<net.minecraft.server.packs.resources.PreparableReloadListener>, net.minecraft.server.packs.resources.SimpleReloadInstance$StateFactory<S>, java.util.concurrent.CompletableFuture<?>);
    protected java.util.concurrent.CompletableFuture<java.util.List<S>> prepareTasks(java.util.concurrent.Executor, java.util.concurrent.Executor, net.minecraft.server.packs.resources.ResourceManager, java.util.List<net.minecraft.server.packs.resources.PreparableReloadListener>, net.minecraft.server.packs.resources.SimpleReloadInstance$StateFactory<S>, java.util.concurrent.CompletableFuture<?>);
    private net.minecraft.server.packs.resources.PreparableReloadListener$PreparationBarrier createBarrierForListener(net.minecraft.server.packs.resources.PreparableReloadListener, java.util.concurrent.CompletableFuture<?>, java.util.concurrent.Executor);
    public java.util.concurrent.CompletableFuture<?> done();
    public float getActualProgress();
    private static int weightProgress(int, int, int);
    public static net.minecraft.server.packs.resources.ReloadInstance create(net.minecraft.server.packs.resources.ResourceManager, java.util.List<net.minecraft.server.packs.resources.PreparableReloadListener>, java.util.concurrent.Executor, java.util.concurrent.Executor, java.util.concurrent.CompletableFuture<net.minecraft.util.Unit>, boolean);
    private static void lambda$prepareTasks$4(net.minecraft.server.packs.resources.PreparableReloadListener$SharedState, net.minecraft.server.packs.resources.PreparableReloadListener);
    private void lambda$prepareTasks$2(java.util.concurrent.Executor, java.lang.Runnable);
    private void lambda$prepareTasks$3(java.lang.Runnable);
    private void lambda$prepareTasks$0(java.util.concurrent.Executor, java.lang.Runnable);
    private void lambda$prepareTasks$1(java.lang.Runnable);
}
```
