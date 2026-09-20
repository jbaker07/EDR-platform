---
type: "interface"
fqcn: "net.minecraft.client.gui.components.Button$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.Button$Builder

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `bounds(IIII)Lnet/minecraft/client/gui/components/Button$Builder;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `bounds(IIII)Lnet/minecraft/client/gui/components/Button$Builder;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `build()Lnet/minecraft/client/gui/components/Button;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `build()Lnet/minecraft/client/gui/components/Button;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `build()Lnet/minecraft/client/gui/components/Button;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `width(I)Lnet/minecraft/client/gui/components/Button$Builder;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.components.Button$Builder {
    private final net.minecraft.network.chat.Component message;
    private final net.minecraft.client.gui.components.Button$OnPress onPress;
    private net.minecraft.client.gui.components.Tooltip tooltip;
    private int x;
    private int y;
    private int width;
    private int height;
    private net.minecraft.client.gui.components.Button$CreateNarration createNarration;
    public net.minecraft.client.gui.components.Button$Builder(net.minecraft.network.chat.Component, net.minecraft.client.gui.components.Button$OnPress);
    public net.minecraft.client.gui.components.Button$Builder pos(int, int);
    public net.minecraft.client.gui.components.Button$Builder width(int);
    public net.minecraft.client.gui.components.Button$Builder size(int, int);
    public net.minecraft.client.gui.components.Button$Builder bounds(int, int, int, int);
    public net.minecraft.client.gui.components.Button$Builder tooltip(net.minecraft.client.gui.components.Tooltip);
    public net.minecraft.client.gui.components.Button$Builder createNarration(net.minecraft.client.gui.components.Button$CreateNarration);
    public net.minecraft.client.gui.components.Button build();
}
```
