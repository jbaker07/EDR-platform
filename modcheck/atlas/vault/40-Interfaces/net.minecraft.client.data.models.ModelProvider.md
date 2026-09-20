---
type: "interface"
fqcn: "net.minecraft.client.data.models.ModelProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.data.models.ModelProvider

System: [[20-Systems/net.minecraft.client.data|net.minecraft.client.data]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `run` | `@Inject at INVOKE Lnet/minecraft/client/data/models/BlockModelGenerators;run()V` | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.data.models.ModelProvider implements net.minecraft.data.DataProvider {
    private final net.minecraft.data.PackOutput$PathProvider blockStatePathProvider;
    private final net.minecraft.data.PackOutput$PathProvider itemInfoPathProvider;
    private final net.minecraft.data.PackOutput$PathProvider modelPathProvider;
    public net.minecraft.client.data.models.ModelProvider(net.minecraft.data.PackOutput);
    public java.util.concurrent.CompletableFuture<?> run(net.minecraft.data.CachedOutput);
    public final java.lang.String getName();
}
```
