---
type: "interface"
fqcn: "net.minecraft.world.InteractionResult$SwingSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.InteractionResult$SwingSource

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `PREDICTEDLnet/minecraft/world/InteractionResult$SwingSource;` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.InteractionResult$SwingSource extends java.lang.Enum<net.minecraft.world.InteractionResult$SwingSource> {
    public static final net.minecraft.world.InteractionResult$SwingSource NONE;
    public static final net.minecraft.world.InteractionResult$SwingSource PREDICTED;
    public static final net.minecraft.world.InteractionResult$SwingSource SERVER_ONLY;
    private static final net.minecraft.world.InteractionResult$SwingSource[] $VALUES;
    public static net.minecraft.world.InteractionResult$SwingSource[] values();
    public static net.minecraft.world.InteractionResult$SwingSource valueOf(java.lang.String);
    private net.minecraft.world.InteractionResult$SwingSource();
    private static net.minecraft.world.InteractionResult$SwingSource[] $values();
    static {};
}
```
