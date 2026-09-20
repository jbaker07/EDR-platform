---
type: "interface"
fqcn: "net.minecraft.data.PackOutput$Target"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.PackOutput$Target

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `DATA_PACKLnet/minecraft/data/PackOutput$Target;` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.data.PackOutput$Target extends java.lang.Enum<net.minecraft.data.PackOutput$Target> {
    public static final net.minecraft.data.PackOutput$Target DATA_PACK;
    public static final net.minecraft.data.PackOutput$Target RESOURCE_PACK;
    public static final net.minecraft.data.PackOutput$Target REPORTS;
    private final java.lang.String directory;
    private static final net.minecraft.data.PackOutput$Target[] $VALUES;
    public static net.minecraft.data.PackOutput$Target[] values();
    public static net.minecraft.data.PackOutput$Target valueOf(java.lang.String);
    private net.minecraft.data.PackOutput$Target(java.lang.String);
    private static net.minecraft.data.PackOutput$Target[] $values();
    static {};
}
```
