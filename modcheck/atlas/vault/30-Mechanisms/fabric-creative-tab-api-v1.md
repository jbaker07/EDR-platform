---
type: "mechanism"
module: "fabric-creative-tab-api-v1"
version: "5.0.21+fcdff87f5d"
sha256: "415e659be69014edac017bc384a96c4f2da5ce8fa2093e7d488593e05f03a0b6"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-creative-tab-api-v1

**Version** `5.0.21+fcdff87f5d` -- **artifact sha256** `415e659be69014edac017bc384a96c4f2da5ce8fa2093e7d488593e05f03a0b6`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-resource-loader-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-creative-tab-api-v1.mixins.json", {"config": "fabric-creative-tab-api-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-creative-tab-api-v1.classtweaker`
- mixin classes: 4 found by annotation, 4 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.creativetab.v1.CreativeModeTabEvents.MODIFY_OUTPUT_ALL|CreativeModeTabEvents.MODIFY_OUTPUT_ALL]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]].`checkTabClicked` | `(Lnet/minecraft/world/item/CreativeModeTab;DD)Z` | name_only | @Inject | HEAD | client | 1000 (default) | `CreativeModeInventoryScreenMixin.isClickInTab` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]].`checkTabHovering` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/item/CreativeModeTab;II)Z` | name_only | @Inject | HEAD | client | 1000 (default) | `CreativeModeInventoryScreenMixin.renderTabTooltipIfHovered` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]].`extractTabButton` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IILnet/minecraft/world/item/CreativeModeTab;)V` | name_only | @Inject | HEAD | client | 1000 (default) | `CreativeModeInventoryScreenMixin.extractTabButton` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]].`init` | `?` | ambiguous | @Inject | INVOKE `Lnet/minecraft/client/gui/components/EditBox;setTextColor(I)V` (exact) | client | 1000 (default) | `CreativeModeInventoryScreenMixin.init` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]].`keyPressed` | `(Lnet/minecraft/client/input/KeyEvent;)Z` | name_only | @Inject | HEAD | client | 1000 (default) | `CreativeModeInventoryScreenMixin.keyPressed` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]].`selectTab` | `(Lnet/minecraft/world/item/CreativeModeTab;)V` | name_only | @Inject | HEAD | client | 1000 (default) | `CreativeModeInventoryScreenMixin.setSelectedTab` |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTab|CreativeModeTab]].`buildContents` | `(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `CreativeModeTabMixin.getStacks` |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]].`buildAllTabContents` | `(Lnet/minecraft/world/item/CreativeModeTab$ItemDisplayParameters;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `CreativeModeTabsMixin.paginateTabs` |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]].`validate` | `()V` | name_only | @Inject | HEAD | both | 1000 (default) | `CreativeModeTabsMixin.deferDuplicateCheck` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.creativetab.v1.FabricCreativeModeInventoryScreen|FabricCreativeModeInventoryScreen]] (interface, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.creativetab.v1.CreativeModeTabEvents|CreativeModeTabEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.creativetab.v1.FabricCreativeModeTab|FabricCreativeModeTab]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.creativetab.v1.FabricCreativeModeTabOutput|FabricCreativeModeTabOutput]] (class, 29 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
