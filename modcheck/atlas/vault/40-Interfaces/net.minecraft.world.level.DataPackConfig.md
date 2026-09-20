---
type: "interface"
fqcn: "net.minecraft.world.level.DataPackConfig"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.DataPackConfig

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/util/List;Ljava/util/List;)V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getDisabled()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getEnabled()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.DataPackConfig {
    public static final net.minecraft.world.level.DataPackConfig DEFAULT;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.DataPackConfig> CODEC;
    private final java.util.List<java.lang.String> enabled;
    private final java.util.List<java.lang.String> disabled;
    public net.minecraft.world.level.DataPackConfig(java.util.List<java.lang.String>, java.util.List<java.lang.String>);
    public java.util.List<java.lang.String> getEnabled();
    public java.util.List<java.lang.String> getDisabled();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static java.util.List lambda$static$2(net.minecraft.world.level.DataPackConfig);
    private static java.util.List lambda$static$1(net.minecraft.world.level.DataPackConfig);
    static {};
}
```
