---
type: "interface"
fqcn: "net.minecraft.server.commands.EnchantCommand"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands.EnchantCommand

System: [[20-Systems/net.minecraft.server.commands|net.minecraft.server.commands]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `enchant` | `@Redirect at INVOKE Lnet/minecraft/world/item/enchantment/Enchantment;canEnchant` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (18, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.commands.EnchantCommand {
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_NOT_LIVING_ENTITY;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_NO_ITEM;
    private static final com.mojang.brigadier.exceptions.DynamicCommandExceptionType ERROR_INCOMPATIBLE;
    private static final com.mojang.brigadier.exceptions.Dynamic2CommandExceptionType ERROR_LEVEL_TOO_HIGH;
    private static final com.mojang.brigadier.exceptions.SimpleCommandExceptionType ERROR_NOTHING_HAPPENED;
    private static final net.minecraft.server.commands.CommandResponseTracker$MessagesWithArgs<net.minecraft.world.entity.Entity, net.minecraft.core.Holder<net.minecraft.world.item.enchantment.Enchantment>, java.lang.Integer> RESPONSE_ENCHANT;
    public net.minecraft.server.commands.EnchantCommand();
    public static void register(com.mojang.brigadier.CommandDispatcher<net.minecraft.commands.CommandSourceStack>, net.minecraft.commands.CommandBuildContext);
    private static int enchant(net.minecraft.commands.CommandSourceStack, java.util.Collection<? extends net.minecraft.world.entity.Entity>, net.minecraft.core.Holder<net.minecraft.world.item.enchantment.Enchantment>, int) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$1(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static int lambda$register$0(com.mojang.brigadier.context.CommandContext) throws com.mojang.brigadier.exceptions.CommandSyntaxException;
    private static net.minecraft.network.chat.Component lambda$static$5(int, int, net.minecraft.core.Holder, java.lang.Integer);
    private static net.minecraft.network.chat.Component lambda$static$4(net.minecraft.world.entity.Entity, int, net.minecraft.core.Holder, java.lang.Integer);
    private static com.mojang.brigadier.Message lambda$static$3(java.lang.Object, java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$2(java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$1(java.lang.Object);
    private static com.mojang.brigadier.Message lambda$static$0(java.lang.Object);
    static {};
}
```
