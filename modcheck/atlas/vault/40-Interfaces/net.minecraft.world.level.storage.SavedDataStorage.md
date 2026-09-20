---
type: "interface"
fqcn: "net.minecraft.world.level.storage.SavedDataStorage"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.SavedDataStorage

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/nio/file/Path;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/c` | exact | invokespecial@81 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@121 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@131 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `computeIfAbsent` | `(Lnet/minecraft/world/level/saveddata/SavedDataType;)Lnet/minecraft/wo` | exact | invokevirtual@44 in `MinecraftServerMixin.initGlobalAttachments` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `computeIfAbsent` | `(Lnet/minecraft/world/level/saveddata/SavedDataType;)Lnet/minecraft/wo` | exact | invokevirtual@32 in `ServerLevelMixin.createAttachmentsPersistentState` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `saveAndJoin` | `()V` | exact | invokevirtual@116 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/level/saveddata/SavedDataType;Lnet/minecraft/wor` | exact | invokevirtual@93 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/level/saveddata/SavedDataType;Lnet/minecraft/wor` | exact | invokevirtual@111 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| wraps | `readTagFromDisk` | `(Ljava/nio/file/Path;Lnet/minecraft/util/datafix/DataFixTypes;I)Lnet/m` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (7 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final cache : Ljava/util/Map;
private final fixerUpper : Lcom/mojang/datafixers/DataFixer;
private final registries : Lnet/minecraft/core/HolderLookup$Provider;
private final dataFolder : Ljava/nio/file/Path;
private pendingWriteFuture : Ljava/util/concurrent/CompletableFuture;
private closed : Z
public <init>(Ljava/nio/file/Path;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/core/HolderLookup$Provider;)V
private getDataFile(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;
public computeIfAbsent(Lnet/minecraft/world/level/saveddata/SavedDataType;)Lnet/minecraft/world/level/saveddata/SavedData;
public get(Lnet/minecraft/world/level/saveddata/SavedDataType;)Lnet/minecraft/world/level/saveddata/SavedData;
private readSavedData(Lnet/minecraft/world/level/saveddata/SavedDataType;)Lnet/minecraft/world/level/saveddata/SavedData;
public set(Lnet/minecraft/world/level/saveddata/SavedDataType;Lnet/minecraft/world/level/saveddata/SavedData;)V
public readTagFromDisk(Ljava/nio/file/Path;Lnet/minecraft/util/datafix/DataFixTypes;I)Lnet/minecraft/nbt/CompoundTag;
private isGzip(Ljava/io/PushbackInputStream;)Z
public scheduleSave()Ljava/util/concurrent/CompletableFuture;
private collectDirtyTagsToSave()Ljava/util/Map;
private encodeUnchecked(Lnet/minecraft/world/level/saveddata/SavedDataType;Lnet/minecraft/world/level/saveddata/SavedData;Lnet/minecraft/resources/RegistryOps;)Lnet/minecraft/nbt/CompoundTag;
private tryWrite(Lnet/minecraft/world/level/saveddata/SavedDataType;Lnet/minecraft/nbt/CompoundTag;)V
public saveAndJoin()V
public close()V
private synthetic lambda$collectDirtyTagsToSave$0(Ljava/util/Map;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/world/level/saveddata/SavedDataType;Ljava/util/Optional;)V
private synthetic lambda$collectDirtyTagsToSave$1(Ljava/util/Map;Lnet/minecraft/world/level/saveddata/SavedDataType;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/world/level/saveddata/SavedData;)V
private synthetic lambda$scheduleSave$3(Ljava/util/Map;Ljava/lang/Object;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$scheduleSave$6(I)[Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$scheduleSave$4(Ljava/util/Map$Entry;)Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$scheduleSave$5(Ljava/util/Map$Entry;)V
private synthetic lambda$scheduleSave$0(IILjava/util/Map;Ljava/lang/Object;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$scheduleSave$2(I)[Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$scheduleSave$1(Ljava/util/List;)V
private static synthetic lambda$readSavedData$0(Lnet/minecraft/world/level/saveddata/SavedDataType;Ljava/lang/String;)V
static <clinit>()V
```
