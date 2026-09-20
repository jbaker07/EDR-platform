---
type: "interface"
fqcn: "net.minecraft.data.recipes.RecipeOutput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.RecipeOutput

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getRecipeIdentifier(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resourc` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRecipeIdentifier(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resourc` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRecipeIdentifier(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resourc` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRecipeIdentifier(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resourc` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (2, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.data.recipes.RecipeOutput extends net.minecraft.data.worldgen.BootstrapContextAccess {
    public abstract void accept(net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>, net.minecraft.world.item.crafting.Recipe<?>, net.minecraft.advancements.AdvancementHolder);
    public abstract net.minecraft.advancements.Advancement$Builder advancement();
}
```
