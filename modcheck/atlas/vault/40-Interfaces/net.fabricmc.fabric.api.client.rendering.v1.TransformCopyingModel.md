---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.rendering.v1.TransformCopyingModel"
module: "fabric-rendering-v1"
sha256: "749427999b4845b129683b1db268a04b524abb6ab351dcaf67cda9a3ab56b5c0"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.TransformCopyingModel

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- kind: class

```java
public static <S, D> net.fabricmc.fabric.api.client.rendering.v1.TransformCopyingModel<S, D> create(net.minecraft.client.model.Model<? super S>, net.minecraft.client.model.Model<? super D>, boolean)
public void setupAnim(com.mojang.datafixers.util.Pair<S, D>)
public void fabric$calculateChildParts(net.minecraft.client.model.geom.ModelPart)
public net.minecraft.client.model.geom.ModelPart getChildPart(java.lang.String)
public void copyTransforms(net.minecraft.client.model.Model<?>)
public void setupAnim(java.lang.Object)
```
