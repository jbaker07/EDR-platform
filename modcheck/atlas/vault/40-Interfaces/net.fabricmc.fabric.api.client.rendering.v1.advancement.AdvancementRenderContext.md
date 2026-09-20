---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.rendering.v1.advancement.AdvancementRenderContext"
module: "fabric-rendering-v1"
sha256: "749427999b4845b129683b1db268a04b524abb6ab351dcaf67cda9a3ab56b5c0"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.advancement.AdvancementRenderContext

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- kind: interface

```java
public abstract net.minecraft.client.gui.GuiGraphicsExtractor graphics()
public abstract net.minecraft.advancements.AdvancementHolder holder()
public abstract net.minecraft.advancements.AdvancementProgress progress()
public net.minecraft.advancements.Advancement advancement()
public net.minecraft.advancements.DisplayInfo display()
public boolean isObtained()
```
