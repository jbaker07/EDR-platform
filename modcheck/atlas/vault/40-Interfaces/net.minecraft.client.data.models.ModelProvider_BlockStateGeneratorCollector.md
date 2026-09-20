---
type: "interface"
fqcn: "net.minecraft.client.data.models.ModelProvider$BlockStateGeneratorCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.data.models.ModelProvider$BlockStateGeneratorCollector

System: [[20-Systems/net.minecraft.client.data|net.minecraft.client.data]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `validate` | `@ModifyArg at INVOKE Ljava/util/stream/Stream;filter(Ljava/util/function/Predica` | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `generatorsLjava/util/Map;` | `` | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
class net.minecraft.client.data.models.ModelProvider$BlockStateGeneratorCollector implements java.util.function.Consumer<net.minecraft.client.data.models.blockstates.BlockModelDefinitionGenerator> {
    private final java.util.Map<net.minecraft.world.level.block.Block, net.minecraft.client.data.models.blockstates.BlockModelDefinitionGenerator> generators;
    private net.minecraft.client.data.models.ModelProvider$BlockStateGeneratorCollector();
    public void accept(net.minecraft.client.data.models.blockstates.BlockModelDefinitionGenerator);
    public void validate();
    public java.util.concurrent.CompletableFuture<?> save(net.minecraft.data.CachedOutput, net.minecraft.data.PackOutput$PathProvider);
    public void accept(java.lang.Object);
    private static java.nio.file.Path lambda$save$0(net.minecraft.data.PackOutput$PathProvider, net.minecraft.world.level.block.Block);
    private static net.minecraft.resources.Identifier lambda$validate$1(net.minecraft.core.Holder$Reference);
    private boolean lambda$validate$0(net.minecraft.core.Holder$Reference);
}
```
