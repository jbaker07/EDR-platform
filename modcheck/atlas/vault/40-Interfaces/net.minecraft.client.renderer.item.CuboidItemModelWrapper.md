---
type: "interface"
fqcn: "net.minecraft.client.renderer.item.CuboidItemModelWrapper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.item.CuboidItemModelWrapper

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `update` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.item.CuboidItemModelWrapper implements net.minecraft.client.renderer.item.ItemModel {
    private final java.util.List<net.minecraft.client.color.item.ItemTintSource> tints;
    private final boolean animated;
    private final net.minecraft.client.resources.model.geometry.ItemQuads itemQuads;
    private final java.util.function.Supplier<org.joml.Vector3fc[]> extents;
    private final net.minecraft.client.renderer.item.ModelRenderProperties properties;
    private final org.joml.Matrix4fc transformation;
    private net.minecraft.client.renderer.item.CuboidItemModelWrapper(java.util.List<net.minecraft.client.color.item.ItemTintSource>, net.minecraft.client.resources.model.geometry.QuadCollection, net.minecraft.client.renderer.item.ModelRenderProperties, org.joml.Matrix4fc);
    public static org.joml.Vector3fc[] computeExtents(java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad>);
    public void update(net.minecraft.client.renderer.item.ItemStackRenderState, net.minecraft.world.item.ItemStack, net.minecraft.client.renderer.item.ItemModelResolver, net.minecraft.world.item.ItemDisplayContext, net.minecraft.client.multiplayer.ClientLevel, net.minecraft.world.entity.ItemOwner, int);
    private static void validateAtlasUsage(java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad>);
    private static boolean hasSpecialAnimatedTexture(net.minecraft.world.item.ItemStack);
    private static org.joml.Vector3fc[] lambda$computeExtents$0(int);
    private static org.joml.Vector3fc[] lambda$new$0(net.minecraft.client.resources.model.geometry.QuadCollection);
}
```
