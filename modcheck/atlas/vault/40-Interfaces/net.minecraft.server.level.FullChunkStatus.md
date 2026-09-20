---
type: "interface"
fqcn: "net.minecraft.server.level.FullChunkStatus"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.FullChunkStatus

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `isOrAfter(Lnet/minecraft/server/level/FullChunkStatus;)Z` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `ordinal()I` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `ordinal()I` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `values()[Lnet/minecraft/server/level/FullChunkStatus;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `values()[Lnet/minecraft/server/level/FullChunkStatus;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `BLOCK_TICKINGLnet/minecraft/server/level/FullChunkStatus;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `ENTITY_TICKINGLnet/minecraft/server/level/FullChunkStatus;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `FULLLnet/minecraft/server/level/FullChunkStatus;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `INACCESSIBLELnet/minecraft/server/level/FullChunkStatus;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.level.FullChunkStatus extends java.lang.Enum<net.minecraft.server.level.FullChunkStatus> {
    public static final net.minecraft.server.level.FullChunkStatus INACCESSIBLE;
    public static final net.minecraft.server.level.FullChunkStatus FULL;
    public static final net.minecraft.server.level.FullChunkStatus BLOCK_TICKING;
    public static final net.minecraft.server.level.FullChunkStatus ENTITY_TICKING;
    private static final net.minecraft.server.level.FullChunkStatus[] $VALUES;
    public static net.minecraft.server.level.FullChunkStatus[] values();
    public static net.minecraft.server.level.FullChunkStatus valueOf(java.lang.String);
    private net.minecraft.server.level.FullChunkStatus();
    public boolean isOrAfter(net.minecraft.server.level.FullChunkStatus);
    private static net.minecraft.server.level.FullChunkStatus[] $values();
    static {};
}
```
