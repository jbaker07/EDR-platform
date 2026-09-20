---
type: "interface"
fqcn: "net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset$1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset$1

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `apply` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (2, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
class net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset$1 implements net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset$SourceProvider {
    net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset$1();
    public <T> net.minecraft.world.level.biome.Climate$ParameterList<T> apply(java.util.function.Function<net.minecraft.resources.ResourceKey<net.minecraft.world.level.biome.Biome>, T>);
}
```
