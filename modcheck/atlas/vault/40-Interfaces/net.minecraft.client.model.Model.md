---
type: "interface"
fqcn: "net.minecraft.client.model.Model"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.model.Model

System: [[20-Systems/net.minecraft.client.model|net.minecraft.client.model]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `root()Lnet/minecraft/client/model/geom/ModelPart;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.model.Model<S> {
    protected final net.minecraft.client.model.geom.ModelPart root;
    protected final java.util.function.Function<net.minecraft.resources.Identifier, net.minecraft.client.renderer.rendertype.RenderType> renderType;
    private final java.util.List<net.minecraft.client.model.geom.ModelPart> allParts;
    public net.minecraft.client.model.Model(net.minecraft.client.model.geom.ModelPart, java.util.function.Function<net.minecraft.resources.Identifier, net.minecraft.client.renderer.rendertype.RenderType>);
    public final java.util.function.Function<net.minecraft.resources.Identifier, net.minecraft.client.renderer.rendertype.RenderType> renderType();
    public final net.minecraft.client.renderer.rendertype.RenderType renderType(net.minecraft.resources.Identifier);
    public final void renderToBuffer(com.mojang.blaze3d.vertex.PoseStack, com.mojang.blaze3d.vertex.VertexConsumer, int, int, int);
    public final net.minecraft.client.model.geom.ModelPart root();
    public final java.util.List<net.minecraft.client.model.geom.ModelPart> allParts();
    public void setupAnim(S);
    public final void resetPose();
}
```
