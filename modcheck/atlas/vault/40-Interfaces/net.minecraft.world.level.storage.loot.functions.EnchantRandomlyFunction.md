---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `lambda$run$1` | `@Redirect at INVOKE Lnet/minecraft/world/item/enchantment/Enchantment;canEnchant` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction extends net.minecraft.world.level.storage.loot.functions.LootItemConditionalFunction {
    private static final org.slf4j.Logger LOGGER;
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction> MAP_CODEC;
    private final java.util.Optional<net.minecraft.core.HolderSet<net.minecraft.world.item.enchantment.Enchantment>> options;
    private final boolean onlyCompatible;
    private final boolean includeAdditionalCostComponent;
    private net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction(java.util.Optional<net.minecraft.core.Holder<net.minecraft.world.level.storage.loot.predicates.LootItemCondition>>, java.util.Optional<net.minecraft.core.HolderSet<net.minecraft.world.item.enchantment.Enchantment>>, boolean, boolean);
    public com.mojang.serialization.MapCodec<net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction> codec();
    public java.util.Set<net.minecraft.util.context.ContextKey<?>> getReferencedContextParams();
    public net.minecraft.world.item.ItemStack run(net.minecraft.world.item.ItemStack, net.minecraft.world.level.storage.loot.LootContext);
    private net.minecraft.world.item.ItemStack enchantItem(net.minecraft.world.item.ItemStack, net.minecraft.core.Holder<net.minecraft.world.item.enchantment.Enchantment>, net.minecraft.world.level.storage.loot.LootContext);
    public static net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction$Builder randomEnchantment();
    public static net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction$Builder randomApplicableEnchantment(net.minecraft.core.HolderGetter<net.minecraft.world.item.enchantment.Enchantment>);
    private static boolean lambda$run$1(boolean, net.minecraft.world.item.ItemStack, net.minecraft.core.Holder);
    private static java.util.stream.Stream lambda$run$0(net.minecraft.world.level.storage.loot.LootContext);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static java.lang.Boolean lambda$static$3(net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction);
    private static java.lang.Boolean lambda$static$2(net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction);
    private static java.util.Optional lambda$static$1(net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction);
    static {};
}
```
