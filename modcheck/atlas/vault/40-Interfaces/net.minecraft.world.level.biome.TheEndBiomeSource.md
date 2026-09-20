---
type: "interface"
fqcn: "net.minecraft.world.level.biome.TheEndBiomeSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.TheEndBiomeSource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<clinit>` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| injects_into | `create` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| injects_into | `create` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| injects_into | `getNoiseBiome` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.biome.TheEndBiomeSource extends net.minecraft.world.level.biome.BiomeSource {
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.level.biome.TheEndBiomeSource> CODEC;
    private final net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> end;
    private final net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> highlands;
    private final net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> midlands;
    private final net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> islands;
    private final net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> barrens;
    public static net.minecraft.world.level.biome.TheEndBiomeSource create(net.minecraft.core.HolderGetter<net.minecraft.world.level.biome.Biome>);
    private net.minecraft.world.level.biome.TheEndBiomeSource(net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>, net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>, net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>, net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>, net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>);
    protected java.util.stream.Stream<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> collectPossibleBiomes();
    protected com.mojang.serialization.MapCodec<net.minecraft.world.level.biome.TheEndBiomeSource> codec();
    public net.minecraft.world.level.biome.BiomeResolver createResolver(net.minecraft.world.level.biome.Climate$Sampler);
    private net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> getNoiseBiome(int, int, int, net.minecraft.world.level.biome.Climate$Sampler);
    private net.minecraft.core.Holder lambda$createResolver$0(net.minecraft.world.level.biome.Climate$Sampler, int, int, int);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
