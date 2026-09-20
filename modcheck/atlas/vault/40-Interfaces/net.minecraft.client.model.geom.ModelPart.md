---
type: "interface"
fqcn: "net.minecraft.client.model.geom.ModelPart"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.model.geom.ModelPart

System: [[20-Systems/net.minecraft.client.model|net.minecraft.client.model]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `xF` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `xRotF` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `xScaleF` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `yF` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `yRotF` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `yScaleF` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `zF` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `zRotF` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `zScaleF` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (44, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.model.geom.ModelPart {
    public static final float DEFAULT_SCALE;
    public float x;
    public float y;
    public float z;
    public float xRot;
    public float yRot;
    public float zRot;
    public float xScale;
    public float yScale;
    public float zScale;
    public boolean visible;
    public boolean skipDraw;
    private final java.util.List<net.minecraft.client.model.geom.ModelPart$Cube> cubes;
    private final java.util.Map<java.lang.String, net.minecraft.client.model.geom.ModelPart> children;
    private net.minecraft.client.model.geom.PartPose initialPose;
    public net.minecraft.client.model.geom.ModelPart(java.util.List<net.minecraft.client.model.geom.ModelPart$Cube>, java.util.Map<java.lang.String, net.minecraft.client.model.geom.ModelPart>);
    public net.minecraft.client.model.geom.PartPose storePose();
    public net.minecraft.client.model.geom.PartPose getInitialPose();
    public void setInitialPose(net.minecraft.client.model.geom.PartPose);
    public void resetPose();
    public void loadPose(net.minecraft.client.model.geom.PartPose);
    public boolean hasChild(java.lang.String);
    public net.minecraft.client.model.geom.ModelPart getChild(java.lang.String);
    public void setPos(float, float, float);
    public void setRotation(float, float, float);
    public void render(com.mojang.blaze3d.vertex.PoseStack, com.mojang.blaze3d.vertex.VertexConsumer, int, int);
    public void render(com.mojang.blaze3d.vertex.PoseStack, com.mojang.blaze3d.vertex.VertexConsumer, int, int, int);
    public void rotateBy(org.joml.Quaternionf);
    public void getExtentsForGui(com.mojang.blaze3d.vertex.PoseStack, java.util.function.Consumer<org.joml.Vector3fc>);
    public void visit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.model.geom.ModelPart$Visitor);
    private void visit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.model.geom.ModelPart$Visitor, java.lang.String);
    public void translateAndRotate(com.mojang.blaze3d.vertex.PoseStack);
    private void compile(com.mojang.blaze3d.vertex.PoseStack$Pose, com.mojang.blaze3d.vertex.VertexConsumer, int, int, int);
    public net.minecraft.client.model.geom.ModelPart$Cube getRandomCube(net.minecraft.util.RandomSource);
    public boolean isEmpty();
    public void offsetPos(org.joml.Vector3f);
    public void offsetRotation(org.joml.Vector3f);
    public void offsetScale(org.joml.Vector3f);
    public java.util.List<net.minecraft.client.model.geom.ModelPart> getAllParts();
    public java.util.function.Function<java.lang.String, net.minecraft.client.model.geom.ModelPart> createPartLookup();
    private void addAllChildren(java.util.function.BiConsumer<java.lang.String, net.minecraft.client.model.geom.ModelPart>);
    private static void lambda$getAllParts$0(java.util.List, java.lang.String, net.minecraft.client.model.geom.ModelPart);
    private static void lambda$visit$0(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.model.geom.ModelPart$Visitor, java.lang.String, java.lang.String, net.minecraft.client.model.geom.ModelPart);
    private static void lambda$getExtentsForGui$0(java.util.function.Consumer, com.mojang.blaze3d.vertex.PoseStack$Pose, java.lang.String, int, net.minecraft.client.model.geom.ModelPart$Cube);
}
```
