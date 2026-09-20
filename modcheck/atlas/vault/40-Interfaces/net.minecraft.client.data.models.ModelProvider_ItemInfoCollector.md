---
type: "interface"
fqcn: "net.minecraft.client.data.models.ModelProvider$ItemInfoCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.data.models.ModelProvider$ItemInfoCollector

System: [[20-Systems/net.minecraft.client.data|net.minecraft.client.data]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `finalizeAndValidate` | `@ModifyArg at INVOKE Ljava/util/stream/Stream;filter(Ljava/util/function/Predica` | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
class net.minecraft.client.data.models.ModelProvider$ItemInfoCollector implements net.minecraft.client.data.models.ItemModelOutput {
    private final java.util.Map<net.minecraft.world.item.Item, net.minecraft.client.renderer.item.ClientItem> itemInfos;
    private final java.util.Map<net.minecraft.world.item.Item, net.minecraft.world.item.Item> copies;
    private net.minecraft.client.data.models.ModelProvider$ItemInfoCollector();
    public void accept(net.minecraft.world.item.Item, net.minecraft.client.renderer.item.ItemModel$Unbaked, net.minecraft.client.renderer.item.ClientItem$Properties);
    private void register(net.minecraft.world.item.Item, net.minecraft.client.renderer.item.ClientItem);
    public void copy(net.minecraft.world.item.Item, net.minecraft.world.item.Item);
    public void finalizeAndValidate();
    public java.util.concurrent.CompletableFuture<?> save(net.minecraft.data.CachedOutput, net.minecraft.data.PackOutput$PathProvider);
    private static java.nio.file.Path lambda$save$0(net.minecraft.data.PackOutput$PathProvider, net.minecraft.world.item.Item);
    private static net.minecraft.resources.Identifier lambda$finalizeAndValidate$3(net.minecraft.core.Holder$Reference);
    private boolean lambda$finalizeAndValidate$2(net.minecraft.core.Holder$Reference);
    private void lambda$finalizeAndValidate$1(net.minecraft.world.item.Item, net.minecraft.world.item.Item);
    private void lambda$finalizeAndValidate$0(net.minecraft.world.item.Item);
}
```
