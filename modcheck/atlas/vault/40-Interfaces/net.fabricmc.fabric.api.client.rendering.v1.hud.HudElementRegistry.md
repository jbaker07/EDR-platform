---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry"
module: "fabric-rendering-v1"
sha256: "749427999b4845b129683b1db268a04b524abb6ab351dcaf67cda9a3ab56b5c0"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- kind: interface

```java
public static void addFirst(net.minecraft.resources.Identifier, net.fabricmc.fabric.api.client.rendering.v1.hud.HudElement)
public static void addLast(net.minecraft.resources.Identifier, net.fabricmc.fabric.api.client.rendering.v1.hud.HudElement)
public static void attachElementBefore(net.minecraft.resources.Identifier, net.minecraft.resources.Identifier, net.fabricmc.fabric.api.client.rendering.v1.hud.HudElement)
public static void attachElementAfter(net.minecraft.resources.Identifier, net.minecraft.resources.Identifier, net.fabricmc.fabric.api.client.rendering.v1.hud.HudElement)
public static void removeElement(net.minecraft.resources.Identifier)
public static void replaceElement(net.minecraft.resources.Identifier, java.util.function.Function<net.fabricmc.fabric.api.client.rendering.v1.hud.HudElement, net.fabricmc.fabric.api.client.rendering.v1.hud.HudElement>)
```
