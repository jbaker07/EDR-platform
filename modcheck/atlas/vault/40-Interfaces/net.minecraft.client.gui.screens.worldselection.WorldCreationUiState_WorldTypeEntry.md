---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.WorldCreationUiState$WorldTypeEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.WorldCreationUiState$WorldTypeEntry

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/core/Holder;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.gui.screens.worldselection.WorldCreationUiState$WorldTypeEntry extends java.lang.Record {
    private final net.minecraft.core.Holder<net.minecraft.world.level.levelgen.presets.WorldPreset> preset;
    private static final net.minecraft.network.chat.Component CUSTOM_WORLD_DESCRIPTION;
    public net.minecraft.client.gui.screens.worldselection.WorldCreationUiState$WorldTypeEntry(net.minecraft.core.Holder<net.minecraft.world.level.levelgen.presets.WorldPreset>);
    public net.minecraft.network.chat.Component describePreset();
    public boolean isAmplified();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.Holder<net.minecraft.world.level.levelgen.presets.WorldPreset> preset();
    private static boolean lambda$isAmplified$0(net.minecraft.resources.ResourceKey);
    private static net.minecraft.network.chat.Component lambda$describePreset$0(net.minecraft.resources.ResourceKey);
    static {};
}
```
