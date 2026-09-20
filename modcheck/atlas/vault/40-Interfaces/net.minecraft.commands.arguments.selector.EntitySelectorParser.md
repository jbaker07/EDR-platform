---
type: "interface"
fqcn: "net.minecraft.commands.arguments.selector.EntitySelectorParser"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.commands.arguments.selector.EntitySelectorParser

System: [[20-Systems/net.minecraft.commands.arguments|net.minecraft.commands.arguments]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/command/v2/FabricEntitySelectorParser`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getCustomFlag` | `(Lnet/minecraft/resources/Identifier;)Z` | inherited_exact | invokevirtual@2 in `EntitySelectorOptionRegistry.lambda$registerNonRepeatable$1` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `setCustomFlag` | `(Lnet/minecraft/resources/Identifier;Z)V` | inherited_exact | invokevirtual@10 in `EntitySelectorOptionRegistry.lambda$registerNonRepeatable$0` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (55 fields, 74 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SYNTAX_SELECTOR_START : C
private static final SYNTAX_OPTIONS_START : C
private static final SYNTAX_OPTIONS_END : C
public static final SYNTAX_OPTIONS_KEY_VALUE_SEPARATOR : C
private static final SYNTAX_OPTIONS_SEPARATOR : C
public static final SYNTAX_NOT : C
public static final SYNTAX_TAG : C
private static final SELECTOR_NEAREST_PLAYER : C
private static final SELECTOR_ALL_PLAYERS : C
private static final SELECTOR_RANDOM_PLAYERS : C
private static final SELECTOR_CURRENT_ENTITY : C
private static final SELECTOR_ALL_ENTITIES : C
private static final SELECTOR_NEAREST_ENTITY : C
public static final ERROR_INVALID_NAME_OR_UUID : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
public static final ERROR_UNKNOWN_SELECTOR_TYPE : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
public static final ERROR_SELECTORS_NOT_ALLOWED : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
public static final ERROR_MISSING_SELECTOR_TYPE : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
public static final ERROR_EXPECTED_END_OF_OPTIONS : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
public static final ERROR_EXPECTED_OPTION_VALUE : Lcom/mojang/brigadier/exceptions/DynamicCommandExceptionType;
public static final ORDER_NEAREST : Ljava/util/function/BiConsumer;
public static final ORDER_FURTHEST : Ljava/util/function/BiConsumer;
public static final ORDER_RANDOM : Ljava/util/function/BiConsumer;
public static final SUGGEST_NOTHING : Ljava/util/function/BiFunction;
private final reader : Lcom/mojang/brigadier/StringReader;
private final allowSelectors : Z
private maxResults : I
private includesEntities : Z
private worldLimited : Z
private distance : Lnet/minecraft/advancements/predicates/MinMaxBounds$Doubles;
private level : Lnet/minecraft/advancements/predicates/MinMaxBounds$Ints;
private x : Ljava/lang/Double;
private y : Ljava/lang/Double;
private z : Ljava/lang/Double;
private deltaX : Ljava/lang/Double;
private deltaY : Ljava/lang/Double;
private deltaZ : Ljava/lang/Double;
private rotX : Lnet/minecraft/advancements/predicates/MinMaxBounds$FloatDegrees;
private rotY : Lnet/minecraft/advancements/predicates/MinMaxBounds$FloatDegrees;
private final predicates : Ljava/util/List;
private order : Ljava/util/function/BiConsumer;
private currentEntity : Z
private playerName : Ljava/lang/String;
private startPosition : I
private entityUUID : Ljava/util/UUID;
private suggestions : Ljava/util/function/BiFunction;
private final nameOption : Lnet/minecraft/commands/arguments/selector/options/InvertableSetOptionState;
private final limitedOption : Lnet/minecraft/commands/arguments/selector/options/SetOnceOptionState;
private final sortedOption : Lnet/minecraft/commands/arguments/selector/options/SetOnceOptionState;
private final gamemodeOption : Lnet/minecraft/commands/arguments/selector/options/InvertableSetOptionState;
private final teamOption : Lnet/minecraft/commands/arguments/selector/options/InvertableSetOptionState;
private type : Lnet/minecraft/world/entity/EntityType;
private final typeOption : Lnet/minecraft/commands/arguments/selector/options/InvertableSetOptionState;
private final scoresOption : Lnet/minecraft/commands/arguments/selector/options/SetOnceOptionState;
private final advancementsOption : Lnet/minecraft/commands/arguments/selector/options/SetOnceOptionState;
private usesSelectors : Z
public <init>(Lcom/mojang/brigadier/StringReader;Z)V
public static allowSelectors(Ljava/lang/Object;)Z
public static allowSelectors(Lnet/minecraft/server/permissions/PermissionSetSupplier;)Z
public getSelector()Lnet/minecraft/commands/arguments/selector/EntitySelector;
private createAabb(DDD)Lnet/minecraft/world/phys/AABB;
private finalizePredicates()V
private createRotationPredicate(Lnet/minecraft/advancements/predicates/MinMaxBounds$FloatDegrees;Lnet/minecraft/util/ToFloatFunction;)Ljava/util/function/Predicate;
protected parseSelector()V
protected parseNameOrUUID()V
protected parseOptions()V
public shouldInvertValue()Z
public isTag()Z
public getReader()Lcom/mojang/brigadier/StringReader;
public addPredicate(Ljava/util/function/Predicate;)V
public setWorldLimited()V
public getDistance()Lnet/minecraft/advancements/predicates/MinMaxBounds$Doubles;
public setDistance(Lnet/minecraft/advancements/predicates/MinMaxBounds$Doubles;)V
public getLevel()Lnet/minecraft/advancements/predicates/MinMaxBounds$Ints;
public setLevel(Lnet/minecraft/advancements/predicates/MinMaxBounds$Ints;)V
public getRotX()Lnet/minecraft/advancements/predicates/MinMaxBounds$FloatDegrees;
public setRotX(Lnet/minecraft/advancements/predicates/MinMaxBounds$FloatDegrees;)V
public getRotY()Lnet/minecraft/advancements/predicates/MinMaxBounds$FloatDegrees;
public setRotY(Lnet/minecraft/advancements/predicates/MinMaxBounds$FloatDegrees;)V
public getX()Ljava/lang/Double;
public getY()Ljava/lang/Double;
public getZ()Ljava/lang/Double;
public setX(D)V
public setY(D)V
public setZ(D)V
public setDeltaX(D)V
public setDeltaY(D)V
public setDeltaZ(D)V
public getDeltaX()Ljava/lang/Double;
public getDeltaY()Ljava/lang/Double;
public getDeltaZ()Ljava/lang/Double;
public setMaxResults(I)V
public setIncludesEntities(Z)V
public getOrder()Ljava/util/function/BiConsumer;
public setOrder(Ljava/util/function/BiConsumer;)V
public parse()Lnet/minecraft/commands/arguments/selector/EntitySelector;
private static fillSelectorSuggestions(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;)V
private suggestNameOrSelector(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
private suggestName(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
private suggestSelector(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
private suggestOpenOptions(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
private suggestOptionsKeyOrClose(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
private suggestOptionsKey(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
private suggestOptionsNextOrClose(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
private suggestEquals(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
public isCurrentEntity()Z
public setSuggestions(Ljava/util/function/BiFunction;)V
public fillSuggestions(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
public nameOption()Lnet/minecraft/commands/arguments/selector/options/InvertableSetOptionState;
public limitedOption()Lnet/minecraft/commands/arguments/selector/options/SetOnceOptionState;
public sortedOption()Lnet/minecraft/commands/arguments/selector/options/SetOnceOptionState;
public gamemodeOption()Lnet/minecraft/commands/arguments/selector/options/InvertableSetOptionState;
public teamOption()Lnet/minecraft/commands/arguments/selector/options/InvertableSetOptionState;
public limitToType(Lnet/minecraft/world/entity/EntityType;)V
public typeOption()Lnet/minecraft/commands/arguments/selector/options/InvertableSetOptionState;
public scoresOption()Lnet/minecraft/commands/arguments/selector/options/SetOnceOptionState;
public advancementsOption()Lnet/minecraft/commands/arguments/selector/options/SetOnceOptionState;
private static synthetic lambda$createRotationPredicate$0(Lnet/minecraft/util/ToFloatFunction;FFLnet/minecraft/world/entity/Entity;)Z
private synthetic lambda$finalizePredicates$0(Lnet/minecraft/world/entity/Entity;)Z
private synthetic lambda$getSelector$1(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
private static synthetic lambda$getSelector$0(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
private static synthetic lambda$static$7(Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;Ljava/util/function/Consumer;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$static$6(Lnet/minecraft/world/phys/Vec3;Ljava/util/List;)V
private static synthetic lambda$static$4(Lnet/minecraft/world/phys/Vec3;Ljava/util/List;)V
private static synthetic lambda$static$5(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;)I
private static synthetic lambda$static$2(Lnet/minecraft/world/phys/Vec3;Ljava/util/List;)V
private static synthetic lambda$static$3(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;)I
private static synthetic lambda$static$1(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
private static synthetic lambda$static$0(Ljava/lang/Object;)Lcom/mojang/brigadier/Message;
static <clinit>()V
```
