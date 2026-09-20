---
type: "interface"
fqcn: "net.minecraft.nbt.NbtAccounter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.NbtAccounter

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `unlimitedHeap()Lnet/minecraft/nbt/NbtAccounter;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `unlimitedHeap()Lnet/minecraft/nbt/NbtAccounter;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (18, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.nbt.NbtAccounter {
    public static final int DEFAULT_NBT_QUOTA;
    public static final int UNCOMPRESSED_NBT_QUOTA;
    private static final int MAX_STACK_DEPTH;
    private final long quota;
    private long usage;
    private final int maxDepth;
    private int depth;
    public net.minecraft.nbt.NbtAccounter(long, int);
    public static net.minecraft.nbt.NbtAccounter create(long);
    public static net.minecraft.nbt.NbtAccounter defaultQuota();
    public static net.minecraft.nbt.NbtAccounter uncompressedQuota();
    public static net.minecraft.nbt.NbtAccounter unlimitedHeap();
    public void accountBytes(long, long);
    public void accountBytes(long);
    public void pushDepth();
    public void popDepth();
    public long getUsage();
    public int getDepth();
}
```
