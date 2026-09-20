---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.rendering.v1.ArmorRenderer"
module: "fabric-rendering-v1"
sha256: "749427999b4845b129683b1db268a04b524abb6ab351dcaf67cda9a3ab56b5c0"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.ArmorRenderer

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- kind: interface

```java
public static void register(net.fabricmc.fabric.api.client.rendering.v1.ArmorRenderer$Factory, net.minecraft.world.level.ItemLike...)
public static void register(net.fabricmc.fabric.api.client.rendering.v1.ArmorRenderer, net.minecraft.world.level.ItemLike...)
public static <S, D> void submitTransformCopyingModel(net.minecraft.client.model.Model<? super S>, S, net.minecraft.client.model.Model<? super D>, D, boolean, net.minecraft.client.renderer.OrderedSubmitNodeCollector, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, int, net.minecraft.client.renderer.texture.TextureAtlasSprite, int)
public static <S, D> void submitTransformCopyingModel(net.minecraft.client.model.Model<? super S>, S, net.minecraft.client.model.Model<? super D>, D, boolean, net.minecraft.client.renderer.OrderedSubmitNodeCollector, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, int)
public abstract void render(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, net.minecraft.world.item.ItemStack, net.minecraft.client.renderer.entity.state.HumanoidRenderState, net.minecraft.world.entity.EquipmentSlot, int, net.minecraft.client.model.HumanoidModel<net.minecraft.client.renderer.entity.state.HumanoidRenderState>)
public default boolean shouldRenderDefaultHeadItem(net.minecraft.world.entity.LivingEntity, net.minecraft.world.item.ItemStack)
```
