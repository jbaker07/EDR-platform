---
type: "interface"
fqcn: "net.minecraft.commands.Commands"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.commands.Commands

System: [[20-Systems/net.minecraft.commands|net.minecraft.commands]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `literal(Ljava/lang/String;)Lcom/mojang/brigadier/builder/LiteralArg` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `performPrefixedCommand(Lnet/minecraft/commands/CommandSourceStack;Ljava/lang/Strin` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at INVOKE Lcom/mojang/brigadier/CommandDispatcher;setConsumer(Lcom/mojan` | both | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| injects_into | `<init>` | `@Inject at INVOKE Lnet/minecraft/server/commands/BanIpCommands;register(Lcom/moj` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (39, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.commands.Commands {
    public static final java.lang.String COMMAND_PREFIX;
    private static final java.lang.ThreadLocal<net.minecraft.commands.execution.ExecutionContext<net.minecraft.commands.CommandSourceStack>> CURRENT_EXECUTION_CONTEXT;
    private static final org.slf4j.Logger LOGGER;
    public static final net.minecraft.server.permissions.PermissionCheck LEVEL_ALL;
    public static final net.minecraft.server.permissions.PermissionCheck LEVEL_MODERATORS;
    public static final net.minecraft.server.permissions.PermissionCheck LEVEL_GAMEMASTERS;
    public static final net.minecraft.server.permissions.PermissionCheck LEVEL_ADMINS;
    public static final net.minecraft.server.permissions.PermissionCheck LEVEL_OWNERS;
    private static final net.minecraft.network.protocol.game.ClientboundCommandsPacket$NodeInspector<net.minecraft.commands.CommandSourceStack> COMMAND_NODE_INSPECTOR;
    private final com.mojang.brigadier.CommandDispatcher<net.minecraft.commands.CommandSourceStack> dispatcher;
    public net.minecraft.commands.Commands(net.minecraft.commands.Commands$CommandSelection, net.minecraft.commands.CommandBuildContext);
    public static <S> com.mojang.brigadier.ParseResults<S> mapSource(com.mojang.brigadier.ParseResults<S>, java.util.function.UnaryOperator<S>);
    public void performPrefixedCommand(net.minecraft.commands.CommandSourceStack, java.lang.String);
    public static java.lang.String trimOptionalPrefix(java.lang.String);
    public void performCommand(com.mojang.brigadier.ParseResults<net.minecraft.commands.CommandSourceStack>, java.lang.String);
    private static com.mojang.brigadier.context.ContextChain<net.minecraft.commands.CommandSourceStack> finishParsing(com.mojang.brigadier.ParseResults<net.minecraft.commands.CommandSourceStack>, java.lang.String, net.minecraft.commands.CommandSourceStack);
    public static void executeCommandInContext(net.minecraft.commands.CommandSourceStack, java.util.function.Consumer<net.minecraft.commands.execution.ExecutionContext<net.minecraft.commands.CommandSourceStack>>);
    public void sendCommands(net.minecraft.server.level.ServerPlayer);
    private static <S> void fillUsableCommands(com.mojang.brigadier.tree.CommandNode<S>, com.mojang.brigadier.tree.CommandNode<S>, S, java.util.Map<com.mojang.brigadier.tree.CommandNode<S>, com.mojang.brigadier.tree.CommandNode<S>>);
    public static com.mojang.brigadier.builder.LiteralArgumentBuilder<net.minecraft.commands.CommandSourceStack> literal(java.lang.String);
    public static <T> com.mojang.brigadier.builder.RequiredArgumentBuilder<net.minecraft.commands.CommandSourceStack, T> argument(java.lang.String, com.mojang.brigadier.arguments.ArgumentType<T>);
    public static java.util.function.Predicate<java.lang.String> createValidator(net.minecraft.commands.Commands$ParseFunction);
    public com.mojang.brigadier.CommandDispatcher<net.minecraft.commands.CommandSourceStack> getDispatcher();
    public static <S> void validateParseResults(com.mojang.brigadier.ParseResults<S>) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    public static <S> com.mojang.brigadier.exceptions.CommandSyntaxException getParseException(com.mojang.brigadier.ParseResults<S>);
    public static net.minecraft.commands.CommandBuildContext createValidationContext(net.minecraft.core.HolderLookup$Provider);
    public static void validate();
    public static <T extends net.minecraft.server.permissions.PermissionSetSupplier> net.minecraft.server.permissions.PermissionProviderCheck<T> hasPermission(net.minecraft.server.permissions.PermissionCheck);
    public static net.minecraft.commands.CommandSourceStack createCompilationContext(net.minecraft.server.permissions.PermissionSet);
    private static java.lang.String lambda$validate$2(com.mojang.brigadier.arguments.ArgumentType);
    private static boolean lambda$validate$1(com.mojang.brigadier.arguments.ArgumentType);
    private static void lambda$validate$0(com.mojang.brigadier.CommandDispatcher, com.mojang.brigadier.tree.CommandNode, com.mojang.brigadier.tree.CommandNode, com.mojang.brigadier.tree.CommandNode, java.util.Collection);
    private static boolean lambda$createValidator$0(net.minecraft.commands.Commands$ParseFunction, java.lang.String);
    private static net.minecraft.network.chat.Style lambda$finishParsing$1(java.lang.String, net.minecraft.network.chat.Style);
    private static com.mojang.brigadier.exceptions.CommandSyntaxException lambda$finishParsing$0(com.mojang.brigadier.ParseResults);
    private static net.minecraft.network.chat.Style lambda$performCommand$2(net.minecraft.network.chat.MutableComponent, net.minecraft.network.chat.Style);
    private static void lambda$performCommand$1(java.lang.String, com.mojang.brigadier.context.ContextChain, net.minecraft.commands.CommandSourceStack, net.minecraft.commands.execution.ExecutionContext);
    private static java.lang.String lambda$performCommand$0(java.lang.String);
    static {};
}
```
