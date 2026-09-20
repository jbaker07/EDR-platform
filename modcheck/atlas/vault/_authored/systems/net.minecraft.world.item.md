---
type: "system_note"
id: "net.minecraft.world.item"
side: "shared_by_design"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Items, item stacks and creative tabs

Package `net.minecraft.world.item` -- generated view: [[20-Systems/net.minecraft.world.item|inventory and hooked types]]

**Responsibility.** Item definitions, ItemStack, enchantment and tooltip logic, creative-mode tabs and the data components attached to stacks (net.minecraft.core.component is the component model itself).

**Side.** shared_by_design

**Threads.** Follows the owning side; item use is initiated on the client and authoritative on the server.

**Persistence.** Item stacks persist as components; a custom component type is registered into the DATA_COMPONENT_TYPE registry.

## Extension points

- Items register into [[30-Mechanisms/Registries|Registries]] ITEM; creative tab contents through [[50-Interactions/events/net.fabricmc.fabric.api.creativetab.v1.CreativeModeTabEvents.MODIFY_OUTPUT_ALL|MODIFY_OUTPUT_ALL]].
- Default components of vanilla items are modifiable through [[50-Interactions/events/net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents.MODIFY|MODIFY]] ([[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]]).
- Item use is intercepted by [[50-Interactions/events/net.fabricmc.fabric.api.event.player.UseItemCallback.EVENT|EVENT]].
- Tooltips on the client through [[50-Interactions/events/net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback.EVENT|EVENT]].

## Evidence

- `extracted/edges.json#callback_of`
- [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]]

