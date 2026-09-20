---
type: "interface"
fqcn: "net.minecraft.data.recipes.BrewingProvider$ContainerTransformation"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.BrewingProvider$ContainerTransformation

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `output()Lnet/minecraft/world/item/Item;` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `reagent()Lnet/minecraft/world/item/Item;` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
final class net.minecraft.data.recipes.BrewingProvider$ContainerTransformation extends java.lang.Record {
    private final net.minecraft.world.item.Item container;
    private final net.minecraft.world.item.Item reagent;
    private final net.minecraft.world.item.Item output;
    private net.minecraft.data.recipes.BrewingProvider$ContainerTransformation(net.minecraft.world.item.Item, net.minecraft.world.item.Item, net.minecraft.world.item.Item);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.item.Item container();
    public net.minecraft.world.item.Item reagent();
    public net.minecraft.world.item.Item output();
}
```
