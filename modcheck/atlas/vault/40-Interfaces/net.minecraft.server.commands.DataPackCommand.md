---
type: "interface"
fqcn: "net.minecraft.server.commands.DataPackCommand"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands.DataPackCommand

System: [[20-Systems/net.minecraft.server.commands|net.minecraft.server.commands]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `getPack` | `@Inject at INVOKE Ljava/util/Collection;contains(Ljava/lang/Object;)Z` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| wraps | `lambda$static$10` | `@Redirect at INVOKE Lnet/minecraft/server/packs/repository/PackRepository;getSel` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (61, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.commands.DataPackCommand {
    private static final org.slf4j.Logger LOGGER;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_UNKNOWN_PACK;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_PACK_ALREADY_ENABLED;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_PACK_ALREADY_DISABLED;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_CANNOT_DISABLE_FEATURE;
    private static final com.mojang.brigadier.exceptions.Dynamic2CommandExceptionType ERROR_PACK_FEATURES_NOT_ENABLED;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_PACK_INVALID_NAME;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_PACK_INVALID_FULL_NAME;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_PACK_ALREADY_EXISTS;
    private static final com.mojang.brigadier.exceptions.Dynamic2CommandExceptionType ERROR_PACK_METADATA_ENCODE_FAILURE;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_PACK_IO_FAILURE;
    private static final com.mojang.brigadier.suggestion.SuggestionProvider<net.minecraft.commands.CommandSourceStack> SELECTED_PACKS;
    private static final com.mojang.brigadier.suggestion.SuggestionProvider<net.minecraft.commands.CommandSourceStack> UNSELECTED_PACKS;
    public net.minecraft.server.commands.DataPackCommand();
    public static void register(com.mojang.brigadier.CommandDispatcher<net.minecraft.commands.CommandSourceStack>, net.minecraft.commands.CommandBuildContext);
    private static int createPack(net.minecraft.commands.CommandSourceStack, java.lang.String, net.minecraft.network.chat.Component) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int enablePack(net.minecraft.commands.CommandSourceStack, net.minecraft.server.packs.repository.Pack, net.minecraft.server.commands.DataPackCommand$Inserter) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int disablePack(net.minecraft.commands.CommandSourceStack, net.minecraft.server.packs.repository.Pack);
    private static int listPacks(net.minecraft.commands.CommandSourceStack);
    private static int listAvailablePacks(net.minecraft.commands.CommandSourceStack);
    private static int listEnabledPacks(net.minecraft.commands.CommandSourceStack);
    private static net.minecraft.server.packs.repository.Pack getPack(com.mojang.brigadier.context.CommandContext<net.minecraft.commands.CommandSourceStack>, java.lang.String, boolean) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static net.minecraft.network.chat.Component lambda$listEnabledPacks$1(java.util.Collection);
    private static net.minecraft.network.chat.Component lambda$listEnabledPacks$2(net.minecraft.server.packs.repository.Pack);
    private static net.minecraft.network.chat.Component lambda$listEnabledPacks$0();
    private static net.minecraft.network.chat.Component lambda$listAvailablePacks$2(java.util.List);
    private static net.minecraft.network.chat.Component lambda$listAvailablePacks$3(net.minecraft.server.packs.repository.Pack);
    private static net.minecraft.network.chat.Component lambda$listAvailablePacks$1();
    private static boolean lambda$listAvailablePacks$0(java.util.Collection, net.minecraft.world.flag.FeatureFlagSet, net.minecraft.server.packs.repository.Pack);
    private static net.minecraft.network.chat.Component lambda$disablePack$0(net.minecraft.server.packs.repository.Pack);
    private static net.minecraft.network.chat.Component lambda$enablePack$0(net.minecraft.server.packs.repository.Pack);
    private static net.minecraft.network.chat.Component lambda$createPack$0(java.lang.String);
    private static int lambda$register$13(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$12(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$11(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$10(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$9(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$7(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static void lambda$register$8(java.util.List, net.minecraft.server.packs.repository.Pack) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$6(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$4(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static void lambda$register$5(com.mojang.brigadier.context.CommandContext, java.util.List, net.minecraft.server.packs.repository.Pack) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$2(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static void lambda$register$3(com.mojang.brigadier.context.CommandContext, java.util.List, net.minecraft.server.packs.repository.Pack) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$0(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static void lambda$register$1(java.util.List, net.minecraft.server.packs.repository.Pack) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static java.util.concurrent.CompletableFuture lambda$static$11(com.mojang.brigadier.context.CommandContext, com.mojang.brigadier.suggestion.SuggestionsBuilder) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static boolean lambda$static$13(java.util.Collection, java.lang.String);
    private static boolean lambda$static$12(net.minecraft.world.flag.FeatureFlagSet, net.minecraft.server.packs.repository.Pack);
    private static java.util.concurrent.CompletableFuture lambda$static$10(com.mojang.brigadier.context.CommandContext, com.mojang.brigadier.suggestion.SuggestionsBuilder) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static com.mojang.brigadier.Message lambda$static$9(java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$8(java.lang.Object, java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$7(java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$6(java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$5(java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$4(java.lang.Object, java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$3(java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$2(java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$1(java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$0(java.lang.Object);
    static {};
}
```
