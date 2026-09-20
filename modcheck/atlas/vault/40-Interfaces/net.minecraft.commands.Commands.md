---
type: "interface"
fqcn: "net.minecraft.commands.Commands"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.commands.Commands

System: [[20-Systems/net.minecraft.commands|net.minecraft.commands]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `literal` | `(Ljava/lang/String;)Lcom/mojang/brigadier/builder/LiteralArgumentBuild` | exact | invokestatic@7 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lcom/mojang/brigadier/builder/LiteralArgumentBuild` | exact | invokestatic@24 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `literal` | `(Ljava/lang/String;)Lcom/mojang/brigadier/builder/LiteralArgumentBuild` | exact | invokestatic@75 in `EnumRuleCommand.register` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `performPrefixedCommand` | `(Lnet/minecraft/commands/CommandSourceStack;Ljava/lang/String;)V` | exact | invokevirtual@9 in `TestServerContextImpl.lambda$runCommand$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/commands/Commands$CommandSelection;Lnet/minecraft/comm` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/commands/Commands$CommandSelection;Lnet/minecraft/comm` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `dispatcher` | `Lcom/mojang/brigadier/CommandDispatcher;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | declared |
| reads | `dispatcher` | `Lcom/mojang/brigadier/CommandDispatcher;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |

## Declared members (10 fields, 29 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final COMMAND_PREFIX : Ljava/lang/String;
private static final CURRENT_EXECUTION_CONTEXT : Ljava/lang/ThreadLocal;
private static final LOGGER : Lorg/slf4j/Logger;
public static final LEVEL_ALL : Lnet/minecraft/server/permissions/PermissionCheck;
public static final LEVEL_MODERATORS : Lnet/minecraft/server/permissions/PermissionCheck;
public static final LEVEL_GAMEMASTERS : Lnet/minecraft/server/permissions/PermissionCheck;
public static final LEVEL_ADMINS : Lnet/minecraft/server/permissions/PermissionCheck;
public static final LEVEL_OWNERS : Lnet/minecraft/server/permissions/PermissionCheck;
private static final COMMAND_NODE_INSPECTOR : Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket$NodeInspector;
private final dispatcher : Lcom/mojang/brigadier/CommandDispatcher;
public <init>(Lnet/minecraft/commands/Commands$CommandSelection;Lnet/minecraft/commands/CommandBuildContext;)V
public static mapSource(Lcom/mojang/brigadier/ParseResults;Ljava/util/function/UnaryOperator;)Lcom/mojang/brigadier/ParseResults;
public performPrefixedCommand(Lnet/minecraft/commands/CommandSourceStack;Ljava/lang/String;)V
public static trimOptionalPrefix(Ljava/lang/String;)Ljava/lang/String;
public performCommand(Lcom/mojang/brigadier/ParseResults;Ljava/lang/String;)V
private static finishParsing(Lcom/mojang/brigadier/ParseResults;Ljava/lang/String;Lnet/minecraft/commands/CommandSourceStack;)Lcom/mojang/brigadier/context/ContextChain;
public static executeCommandInContext(Lnet/minecraft/commands/CommandSourceStack;Ljava/util/function/Consumer;)V
public sendCommands(Lnet/minecraft/server/level/ServerPlayer;)V
private static fillUsableCommands(Lcom/mojang/brigadier/tree/CommandNode;Lcom/mojang/brigadier/tree/CommandNode;Ljava/lang/Object;Ljava/util/Map;)V
public static literal(Ljava/lang/String;)Lcom/mojang/brigadier/builder/LiteralArgumentBuilder;
public static argument(Ljava/lang/String;Lcom/mojang/brigadier/arguments/ArgumentType;)Lcom/mojang/brigadier/builder/RequiredArgumentBuilder;
public static createValidator(Lnet/minecraft/commands/Commands$ParseFunction;)Ljava/util/function/Predicate;
public getDispatcher()Lcom/mojang/brigadier/CommandDispatcher;
public static validateParseResults(Lcom/mojang/brigadier/ParseResults;)V
public static getParseException(Lcom/mojang/brigadier/ParseResults;)Lcom/mojang/brigadier/exceptions/CommandSyntaxException;
public static createValidationContext(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/commands/CommandBuildContext;
public static validate()V
public static hasPermission(Lnet/minecraft/server/permissions/PermissionCheck;)Lnet/minecraft/server/permissions/PermissionProviderCheck;
public static createCompilationContext(Lnet/minecraft/server/permissions/PermissionSet;)Lnet/minecraft/commands/CommandSourceStack;
private static synthetic lambda$validate$2(Lcom/mojang/brigadier/arguments/ArgumentType;)Ljava/lang/String;
private static synthetic lambda$validate$1(Lcom/mojang/brigadier/arguments/ArgumentType;)Z
private static synthetic lambda$validate$0(Lcom/mojang/brigadier/CommandDispatcher;Lcom/mojang/brigadier/tree/CommandNode;Lcom/mojang/brigadier/tree/CommandNode;Lcom/mojang/brigadier/tree/CommandNode;Ljava/util/Collection;)V
private static synthetic lambda$createValidator$0(Lnet/minecraft/commands/Commands$ParseFunction;Ljava/lang/String;)Z
private static synthetic lambda$finishParsing$1(Ljava/lang/String;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private static synthetic lambda$finishParsing$0(Lcom/mojang/brigadier/ParseResults;)Lcom/mojang/brigadier/exceptions/CommandSyntaxException;
private static synthetic lambda$performCommand$2(Lnet/minecraft/network/chat/MutableComponent;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private static synthetic lambda$performCommand$1(Ljava/lang/String;Lcom/mojang/brigadier/context/ContextChain;Lnet/minecraft/commands/CommandSourceStack;Lnet/minecraft/commands/execution/ExecutionContext;)V
private static synthetic lambda$performCommand$0(Ljava/lang/String;)Ljava/lang/String;
static <clinit>()V
```
