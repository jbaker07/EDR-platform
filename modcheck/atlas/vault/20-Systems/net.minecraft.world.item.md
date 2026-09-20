---
type: "system"
package: "net.minecraft.world.item"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item

Analyst note: [[_authored/systems/net.minecraft.world.item|Items, item stacks and creative tabs]]

460 classes in the jar. Hooked types: 31

- [[40-Interfaces/net.minecraft.world.item.BlockItem|BlockItem]] -- calls:2 -- by fabric-data-generation-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.item.BucketItem|BucketItem]] -- injects_into:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTab|CreativeModeTab]] -- calls:7, injects_into:1 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTab_Builder|CreativeModeTab$Builder]] -- calls:3 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTab_ItemDisplayParameters|CreativeModeTab$ItemDisplayParameters]] -- calls:2 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTab_Row|CreativeModeTab$Row]] -- reads:2 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] -- calls:5, injects_into:2, reads:16 -- by fabric-creative-tab-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.item.HoneycombItem|HoneycombItem]] -- injects_into:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.item.Item|Item]] -- calls:6, injects_into:1 -- by fabric-item-api-v1, fabric-recipe-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]] -- calls:40, injects_into:5, reads:1 -- by fabric-api-lookup-api-v1, fabric-content-registries-v0, fabric-events-interaction-v0, fabric-item-api-v1, fabric-recipe-api-v1, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.ItemStackTemplate|ItemStackTemplate]] -- calls:2 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.Items|Items]] -- reads:5 -- by fabric-data-generation-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.item.alchemy.PotionContents|PotionContents]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.alchemy.Potions|Potions]] -- reads:1 -- by fabric-data-generation-api-v1
- [[40-Interfaces/net.minecraft.world.item.component.BundleContents|BundleContents]] -- calls:3 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.component.BundleContents_Mutable|BundleContents$Mutable]] -- calls:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.item.component.CustomData|CustomData]] -- calls:2 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.CraftingInput|CraftingInput]] -- calls:3 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.CraftingRecipe|CraftingRecipe]] -- wraps:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]] -- calls:20, injects_into:5 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.Recipe|Recipe]] -- calls:3 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeHolder|RecipeHolder]] -- calls:4 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeInput|RecipeInput]] -- calls:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeManager|RecipeManager]] -- injects_into:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.RecipeMap|RecipeMap]] -- calls:4 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.ShapelessRecipe|ShapelessRecipe]] -- injects_into:2 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.display.SlotDisplay_Composite|SlotDisplay$Composite]] -- calls:3 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.crafting.display.SlotDisplay_ItemStackSlotDisplay|SlotDisplay$ItemStackSlotDisplay]] -- calls:2 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment|Enchantment]] -- calls:6 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.item.enchantment.Enchantment_Builder|Enchantment$Builder]] -- calls:1, injects_into:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.item.enchantment.EnchantmentHelper|EnchantmentHelper]] -- wraps:1 -- by fabric-item-api-v1
