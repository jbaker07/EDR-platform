---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.packs.PackSelectionModel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.packs.PackSelectionModel

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `findNewPacks` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.packs.PackSelectionModel {
    private final net.minecraft.server.packs.repository.PackRepository repository;
    private final java.util.List<net.minecraft.server.packs.repository.Pack> selected;
    private final java.util.List<net.minecraft.server.packs.repository.Pack> unselected;
    private final java.util.function.Function<net.minecraft.server.packs.repository.Pack, net.minecraft.resources.Identifier> iconGetter;
    private final java.util.function.Consumer<net.minecraft.client.gui.screens.packs.PackSelectionModel$EntryBase> onListChanged;
    private final java.util.function.Consumer<net.minecraft.server.packs.repository.PackRepository> output;
    public net.minecraft.client.gui.screens.packs.PackSelectionModel(java.util.function.Consumer<net.minecraft.client.gui.screens.packs.PackSelectionModel$EntryBase>, java.util.function.Function<net.minecraft.server.packs.repository.Pack, net.minecraft.resources.Identifier>, net.minecraft.server.packs.repository.PackRepository, java.util.function.Consumer<net.minecraft.server.packs.repository.PackRepository>);
    public java.util.stream.Stream<net.minecraft.client.gui.screens.packs.PackSelectionModel$Entry> getUnselected();
    public java.util.stream.Stream<net.minecraft.client.gui.screens.packs.PackSelectionModel$Entry> getSelected();
    private void updateRepoSelectedList();
    public void commit();
    public void findNewPacks();
    private net.minecraft.client.gui.screens.packs.PackSelectionModel$Entry lambda$getSelected$0(net.minecraft.server.packs.repository.Pack);
    private net.minecraft.client.gui.screens.packs.PackSelectionModel$Entry lambda$getUnselected$0(net.minecraft.server.packs.repository.Pack);
}
```
