---
type: "interface"
fqcn: "net.minecraft.util.Util"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.Util

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `backgroundExecutor` | `()Lnet/minecraft/TracingExecutor;` | exact | invokestatic@86 in `FabricDataGenHelper.runInternal` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `backgroundExecutor` | `()Lnet/minecraft/TracingExecutor;` | exact | invokestatic@100 in `FabricDataGenHelper.runInternal` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getPlatform` | `()Lnet/minecraft/util/Util$OS;` | exact | invokestatic@10 in `TestInputImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getPlatform` | `()Lnet/minecraft/util/Util$OS;` | exact | invokestatic@12 in `DedicatedServerImplUtil.lambda$static$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `make` | `(Ljava/lang/Object;Ljava/util/function/Consumer;)Ljava/lang/Object;` | exact | invokestatic@20 in `DedicatedServerImplUtil.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `makeDescriptionId` | `(Ljava/lang/String;Lnet/minecraft/resources/Identifier;)Ljava/lang/Str` | exact | invokestatic@16 in `SoundTypeBuilder.of` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `makeDescriptionId` | `(Ljava/lang/String;Lnet/minecraft/resources/Identifier;)Ljava/lang/Str` | exact | invokestatic@7 in `FabricLanguageProvider$TranslationBuilder.addEnchantment` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `makeDescriptionId` | `(Ljava/lang/String;Lnet/minecraft/resources/Identifier;)Ljava/lang/Str` | exact | invokestatic@7 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `makeDescriptionId` | `(Ljava/lang/String;Lnet/minecraft/resources/Identifier;)Ljava/lang/Str` | exact | invokestatic@48 in `FluidVariantAttributeHandler.getName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `sequence` | `(Ljava/util/List;)Ljava/util/concurrent/CompletableFuture;` | exact | invokestatic@99 in `ModelLoadingPluginManager.preparePlugins` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| reads | `NIL_UUID` | `Ljava/util/UUID;` | exact | getstatic@27 in `CommandSourceStackMixin.<init>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `NIL_UUID` | `Ljava/util/UUID;` | exact | getstatic@134 in `CommandSourceStackMixin.storeOriginalSource` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (19 fields, 146 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DEFAULT_MAX_THREADS : I
private static final DEFAULT_SAFE_FILE_OPERATION_RETRIES : I
private static final MAX_THREADS_SYSTEM_PROPERTY : Ljava/lang/String;
private static final BACKGROUND_EXECUTOR : Lnet/minecraft/TracingExecutor;
private static final IO_POOL : Lnet/minecraft/TracingExecutor;
private static final DOWNLOAD_POOL : Lnet/minecraft/TracingExecutor;
private static final FILENAME_DATE_TIME_FORMATTER : Ljava/time/format/DateTimeFormatter;
public static final LINEAR_LOOKUP_THRESHOLD : I
private static final ALLOWED_UNTRUSTED_LINK_PROTOCOLS : Ljava/util/Set;
public static final NANOS_PER_MILLI : J
public static final MILLIS_PER_SECOND : J
private static timeSource : Lnet/minecraft/util/TimeSource$NanoTimeSource;
private static final INDIRECT_TIME_SOURCE : Lnet/minecraft/util/TimeSource$NanoTimeSource;
public static final TICKER : Lcom/google/common/base/Ticker;
public static final NIL_UUID : Ljava/util/UUID;
public static final ZIP_FILE_SYSTEM_PROVIDER : Ljava/nio/file/spi/FileSystemProvider;
public static final CONTROL_CHARACTER_ESCAPER : Lcom/google/common/escape/Escaper;
private static thePauser : Ljava/util/function/Consumer;
public <init>()V
public static toMap()Ljava/util/stream/Collector;
public static toMutableList()Ljava/util/stream/Collector;
public static getPropertyName(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/Object;)Ljava/lang/String;
public static makeDescriptionId(Ljava/lang/String;Lnet/minecraft/resources/Identifier;)Ljava/lang/String;
public static timeSource()Lnet/minecraft/util/TimeSource$NanoTimeSource;
public static setTimeSource(Lnet/minecraft/util/TimeSource$NanoTimeSource;)V
public static shutdownTimeSource()V
public static getMillis()J
public static getNanos()J
public static getEpochMillis()J
public static toMillis(D)J
public static getFilenameFormattedDateTime()Ljava/lang/String;
private static makeExecutor(Ljava/lang/String;)Lnet/minecraft/TracingExecutor;
public static maxAllowedExecutorThreads()I
private static getMaxThreads()I
public static backgroundExecutor()Lnet/minecraft/TracingExecutor;
public static ioPool()Lnet/minecraft/TracingExecutor;
public static nonCriticalIoPool()Lnet/minecraft/TracingExecutor;
public static shutdownExecutors()V
private static makeIoExecutor(Ljava/lang/String;Z)Lnet/minecraft/TracingExecutor;
public static throwAsRuntime(Ljava/lang/Throwable;)V
private static onThreadException(Ljava/lang/Thread;Ljava/lang/Throwable;)V
public static fetchChoiceType(Lcom/mojang/datafixers/DSL$TypeReference;Ljava/lang/String;)Lcom/mojang/datafixers/types/Type;
private static doFetchChoiceType(Lcom/mojang/datafixers/DSL$TypeReference;Ljava/lang/String;)Lcom/mojang/datafixers/types/Type;
public static runNamed(Ljava/lang/Runnable;Ljava/lang/String;)V
public static getRegisteredName(Lnet/minecraft/core/Registry;Ljava/lang/Object;)Ljava/lang/String;
public static allOf()Ljava/util/function/Predicate;
public static allOf(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static allOf(Ljava/util/function/Predicate;Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static allOf(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static allOf(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static allOf(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static allOf([Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static allOf(Ljava/util/List;)Ljava/util/function/Predicate;
public static anyOf()Ljava/util/function/Predicate;
public static anyOf(Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static anyOf(Ljava/util/function/Predicate;Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static anyOf(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static anyOf(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static anyOf(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static anyOf([Ljava/util/function/Predicate;)Ljava/util/function/Predicate;
public static anyOf(Ljava/util/List;)Ljava/util/function/Predicate;
public static isSymmetrical(IILjava/util/List;)Z
public static growByHalf(II)I
public static localizedDateFormatter(Ljava/time/format/FormatStyle;)Ljava/time/format/DateTimeFormatter;
public static dumpThreadInfo()[Ljava/lang/management/ThreadInfo;
public static getPlatform()Lnet/minecraft/util/Util$OS;
public static isAarch64()Z
public static isAppleSiliconMac(Ljava/lang/String;)Z
public static parseAndValidateUntrustedUri(Ljava/lang/String;)Ljava/net/URI;
public static findNextInIterable(Ljava/lang/Iterable;Ljava/lang/Object;)Ljava/lang/Object;
public static findPreviousInIterable(Ljava/lang/Iterable;Ljava/lang/Object;)Ljava/lang/Object;
public static make(Ljava/util/function/Supplier;)Ljava/lang/Object;
public static make(Ljava/lang/Object;Ljava/util/function/Consumer;)Ljava/lang/Object;
public static makeEnumMap(Ljava/lang/Class;Ljava/util/function/Function;)Ljava/util/Map;
public static mapValues(Ljava/util/Map;Ljava/util/function/Function;)Ljava/util/Map;
public static mapValuesLazy(Ljava/util/Map;Lcom/google/common/base/Function;)Ljava/util/Map;
public static allOfEnumExcept(Ljava/lang/Enum;)Ljava/util/Set;
public static sequence(Ljava/util/List;)Ljava/util/concurrent/CompletableFuture;
public static sequenceFailFast(Ljava/util/List;)Ljava/util/concurrent/CompletableFuture;
public static sequenceFailFastAndCancel(Ljava/util/List;)Ljava/util/concurrent/CompletableFuture;
private static fallibleSequence(Ljava/util/List;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
public static ifElse(Ljava/util/Optional;Ljava/util/function/Consumer;Ljava/lang/Runnable;)Ljava/util/Optional;
public static name(Ljava/util/function/Supplier;Ljava/util/function/Supplier;)Ljava/util/function/Supplier;
public static name(Ljava/lang/Runnable;Ljava/util/function/Supplier;)Ljava/lang/Runnable;
public static logAndPauseIfInIde(Ljava/lang/String;)V
public static logAndPauseIfInIde(Ljava/lang/String;Ljava/lang/Throwable;)V
public static pauseInIde(Ljava/lang/Throwable;)Ljava/lang/Throwable;
public static setPause(Ljava/util/function/Consumer;)V
private static doPause(Ljava/lang/String;)V
public static describeError(Ljava/lang/Throwable;)Ljava/lang/String;
public static getRandom([Ljava/lang/Object;Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;
public static getRandom([ILnet/minecraft/util/RandomSource;)I
public static getRandom(Ljava/util/List;Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;
public static getRandomSafe(Ljava/util/List;Lnet/minecraft/util/RandomSource;)Ljava/util/Optional;
private static createRenamer(Ljava/nio/file/Path;Ljava/nio/file/Path;[Ljava/nio/file/CopyOption;)Ljava/util/function/BooleanSupplier;
private static createDeleter(Ljava/nio/file/Path;)Ljava/util/function/BooleanSupplier;
private static createFileDeletedCheck(Ljava/nio/file/Path;)Ljava/util/function/BooleanSupplier;
private static createFileCreatedCheck(Ljava/nio/file/Path;)Ljava/util/function/BooleanSupplier;
private static executeInSequence([Ljava/util/function/BooleanSupplier;)Z
private static runWithRetries(ILjava/lang/String;[Ljava/util/function/BooleanSupplier;)Z
public static safeMoveFile(Ljava/nio/file/Path;Ljava/nio/file/Path;[Ljava/nio/file/CopyOption;)Z
public static safeReplaceFile(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/nio/file/Path;)V
public static safeReplaceOrMoveFile(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/nio/file/Path;Z)Z
public static offsetByCodepoints(Ljava/lang/String;II)I
public static prefix(Ljava/lang/String;Ljava/util/function/Consumer;)Ljava/util/function/Consumer;
public static fixedSize(Ljava/util/stream/IntStream;I)Lcom/mojang/serialization/DataResult;
public static fixedSize(Ljava/util/stream/LongStream;I)Lcom/mojang/serialization/DataResult;
public static fixedSize(Ljava/util/List;I)Lcom/mojang/serialization/DataResult;
public static startTimerHackThread()V
public static copyBetweenDirs(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/nio/file/Path;)V
public static sanitizeName(Ljava/lang/String;Lnet/minecraft/CharPredicate;)Ljava/lang/String;
public static singleKeyCache(Ljava/util/function/Function;)Lnet/minecraft/util/SingleKeyCache;
public static memoize(Ljava/util/function/Function;)Ljava/util/function/Function;
public static memoize(Ljava/util/function/BiFunction;)Ljava/util/function/BiFunction;
public static toShuffledList(Ljava/util/stream/Stream;Lnet/minecraft/util/RandomSource;)Ljava/util/List;
public static toShuffledList(Ljava/util/stream/IntStream;Lnet/minecraft/util/RandomSource;)Lit/unimi/dsi/fastutil/ints/IntArrayList;
public static shuffledCopy([Ljava/lang/Object;Lnet/minecraft/util/RandomSource;)Ljava/util/List;
public static shuffledCopy(Lit/unimi/dsi/fastutil/objects/ObjectArrayList;Lnet/minecraft/util/RandomSource;)Ljava/util/List;
public static shuffle(Ljava/util/List;Lnet/minecraft/util/RandomSource;)V
public static blockUntilDone(Ljava/util/function/Function;)Ljava/util/concurrent/CompletableFuture;
public static blockUntilDone(Ljava/util/function/Function;Ljava/util/function/Predicate;)Ljava/lang/Object;
public static createIndexLookup(Ljava/util/List;)Ljava/util/function/ToIntFunction;
public static createIndexIdentityLookup(Ljava/util/List;)Ljava/util/function/ToIntFunction;
public static writeAndReadTypedOrThrow(Lcom/mojang/datafixers/Typed;Lcom/mojang/datafixers/types/Type;Ljava/util/function/UnaryOperator;)Lcom/mojang/datafixers/Typed;
public static readTypedOrThrow(Lcom/mojang/datafixers/types/Type;Lcom/mojang/serialization/Dynamic;)Lcom/mojang/datafixers/Typed;
public static readTypedOrThrow(Lcom/mojang/datafixers/types/Type;Lcom/mojang/serialization/Dynamic;Z)Lcom/mojang/datafixers/Typed;
public static copyAndAdd(Ljava/util/List;Ljava/lang/Object;)Ljava/util/List;
public static copyAndAdd(Ljava/util/List;[Ljava/lang/Object;)Ljava/util/List;
public static copyAndAdd(Ljava/lang/Object;Ljava/util/List;)Ljava/util/List;
public static join(Ljava/util/Collection;Ljava/util/Collection;)Ljava/util/List;
public static join([Ljava/util/Collection;)Ljava/util/List;
public static copyAndPut(Ljava/util/Map;Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/Map;
private static synthetic lambda$sanitizeName$0(Lnet/minecraft/CharPredicate;I)Ljava/lang/String;
private static synthetic lambda$fixedSize$2(I)Ljava/lang/String;
private static synthetic lambda$fixedSize$1(I)Ljava/lang/String;
private static synthetic lambda$fixedSize$0(I)Ljava/lang/String;
private static synthetic lambda$prefix$0(Ljava/util/function/Consumer;Ljava/lang/String;Ljava/lang/String;)V
private static synthetic lambda$fallibleSequence$1(Lit/unimi/dsi/fastutil/objects/ObjectArrayList;Ljava/lang/Void;)Ljava/util/List;
private static synthetic lambda$fallibleSequence$0(Ljava/util/function/Consumer;Lit/unimi/dsi/fastutil/objects/ObjectArrayList;ILjava/lang/Object;Ljava/lang/Throwable;)V
private static synthetic lambda$sequenceFailFastAndCancel$0(Ljava/util/concurrent/CompletableFuture;Ljava/util/List;Ljava/lang/Throwable;)V
private static synthetic lambda$sequence$0(Ljava/util/List;Ljava/lang/Void;)Ljava/util/List;
private static synthetic lambda$mapValues$0(Ljava/util/function/Function;Ljava/util/Map$Entry;)Ljava/lang/Object;
private static synthetic lambda$anyOf$6(I)[Ljava/util/function/Predicate;
private static synthetic lambda$anyOf$5([Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$anyOf$4(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$anyOf$3(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$anyOf$2(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$anyOf$1(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$anyOf$0(Ljava/lang/Object;)Z
private static synthetic lambda$allOf$6(I)[Ljava/util/function/Predicate;
private static synthetic lambda$allOf$5([Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$allOf$4(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$allOf$3(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$allOf$2(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$allOf$1(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Ljava/lang/Object;)Z
private static synthetic lambda$allOf$0(Ljava/lang/Object;)Z
private static synthetic lambda$makeIoExecutor$0(Ljava/lang/String;Ljava/util/concurrent/atomic/AtomicInteger;ZLjava/lang/Runnable;)Ljava/lang/Thread;
private static synthetic lambda$makeExecutor$0(Ljava/lang/String;Ljava/util/concurrent/atomic/AtomicInteger;Ljava/util/concurrent/ForkJoinPool;)Ljava/util/concurrent/ForkJoinWorkerThread;
private static synthetic lambda$static$4(Ljava/lang/String;)V
private static synthetic lambda$static$3(Lcom/google/common/escape/Escapers$Builder;)V
private static synthetic lambda$static$2()Ljava/lang/IllegalStateException;
private static synthetic lambda$static$1(Ljava/nio/file/spi/FileSystemProvider;)Z
private static synthetic lambda$static$0()J
static <clinit>()V
```
