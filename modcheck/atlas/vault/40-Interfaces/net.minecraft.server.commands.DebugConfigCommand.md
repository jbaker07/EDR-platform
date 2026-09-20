---
type: "interface"
fqcn: "net.minecraft.server.commands.DebugConfigCommand"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands.DebugConfigCommand

System: [[20-Systems/net.minecraft.server.commands|net.minecraft.server.commands]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `register(Lcom/mojang/brigadier/CommandDispatcher;Lnet/minecraft/comm` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `unconfig` | `@Redirect at INVOKE Lnet/minecraft/server/network/ServerConfigurationPacketListe` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.commands.DebugConfigCommand {
    public net.minecraft.server.commands.DebugConfigCommand();
    public static void register(com.mojang.brigadier.CommandDispatcher<net.minecraft.commands.CommandSourceStack>, net.minecraft.commands.CommandBuildContext);
    private static java.lang.Iterable<java.lang.String> getUuidsInConfig(net.minecraft.server.MinecraftServer);
    private static int config(net.minecraft.commands.CommandSourceStack, net.minecraft.server.level.ServerPlayer);
    private static net.minecraft.server.network.ServerConfigurationPacketListenerImpl findConfigPlayer(net.minecraft.server.MinecraftServer, java.util.UUID);
    private static int unconfig(net.minecraft.commands.CommandSourceStack, java.util.UUID);
    private static int showDialog(net.minecraft.commands.CommandSourceStack, java.util.UUID, net.minecraft.core.Holder<net.minecraft.server.dialog.Dialog>);
    private static net.minecraft.network.chat.Component lambda$config$0(com.mojang.authlib.GameProfile);
    private static int lambda$register$4(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static java.util.concurrent.CompletableFuture lambda$register$3(com.mojang.brigadier.context.CommandContext, com.mojang.brigadier.suggestion.SuggestionsBuilder) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$2(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static java.util.concurrent.CompletableFuture lambda$register$1(com.mojang.brigadier.context.CommandContext, com.mojang.brigadier.suggestion.SuggestionsBuilder) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$0(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
}
```
