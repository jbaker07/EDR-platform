---
type: "interface"
fqcn: "net.minecraft.world.level.biome.Climate$ParameterList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.Climate$ParameterList

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/util/List;)V` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `values()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.biome.Climate$ParameterList<T> {
    private final java.util.List<com.mojang.datafixers.util.Pair<net.minecraft.world.level.biome.Climate$ParameterPoint, T>> values;
    private final net.minecraft.world.level.biome.Climate$RTree<T> index;
    public static <T> com.mojang.serialization.Codec<net.minecraft.world.level.biome.Climate$ParameterList<T>> codec(com.mojang.serialization.MapCodec<T>);
    public net.minecraft.world.level.biome.Climate$ParameterList(java.util.List<com.mojang.datafixers.util.Pair<net.minecraft.world.level.biome.Climate$ParameterPoint, T>>);
    private net.minecraft.world.level.biome.Climate$ParameterList(java.util.List<com.mojang.datafixers.util.Pair<net.minecraft.world.level.biome.Climate$ParameterPoint, T>>, int);
    public net.minecraft.world.level.biome.Climate$ParameterList<T> rebuildWithChildrenPerNode(int);
    public java.util.List<com.mojang.datafixers.util.Pair<net.minecraft.world.level.biome.Climate$ParameterPoint, T>> values();
    public T findValue(net.minecraft.world.level.biome.Climate$TargetPoint);
    public T findValueBruteForce(net.minecraft.world.level.biome.Climate$TargetPoint);
    public T findValueIndex(net.minecraft.world.level.biome.Climate$TargetPoint);
    protected T findValueIndex(net.minecraft.world.level.biome.Climate$TargetPoint, net.minecraft.world.level.biome.Climate$DistanceMetric<T>);
    private static com.mojang.datafixers.kinds.App lambda$codec$0(com.mojang.serialization.MapCodec, com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
}
```
