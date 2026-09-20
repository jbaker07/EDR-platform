---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.MultiPackResourceManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.MultiPackResourceManager

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.packs.resources.MultiPackResourceManager implements net.minecraft.server.packs.resources.CloseableResourceManager {
    private static final org.slf4j.Logger LOGGER;
    private final java.util.Map<java.lang.String, net.minecraft.server.packs.resources.FallbackResourceManager> namespacedManagers;
    private final java.util.List<net.minecraft.server.packs.PackResources> packs;
    public net.minecraft.server.packs.resources.MultiPackResourceManager(net.minecraft.server.packs.PackType, java.util.List<net.minecraft.server.packs.PackResources>);
    private net.minecraft.server.packs.resources.ResourceFilterSection getPackFilterSection(net.minecraft.server.packs.PackResources);
    public java.util.Set<java.lang.String> getNamespaces();
    public java.util.Optional<net.minecraft.server.packs.resources.Resource> getResource(net.minecraft.resources.Identifier);
    public java.util.List<net.minecraft.server.packs.resources.Resource> getResourceStack(net.minecraft.resources.Identifier);
    public java.util.Map<net.minecraft.resources.Identifier, net.minecraft.server.packs.resources.Resource> listResources(java.lang.String, net.minecraft.server.packs.resources.ResourceManager$Selector);
    public java.util.Map<net.minecraft.resources.Identifier, java.util.List<net.minecraft.server.packs.resources.Resource>> listResourceStacks(java.lang.String, net.minecraft.server.packs.resources.ResourceManager$Selector);
    private static void checkTrailingDirectoryPath(java.lang.String);
    public java.util.stream.Stream<net.minecraft.server.packs.PackResources> listPacks();
    public void close();
    private static boolean lambda$new$1(net.minecraft.server.packs.resources.ResourceFilterSection, net.minecraft.resources.Identifier);
    private static java.util.stream.Stream lambda$new$0(net.minecraft.server.packs.PackType, net.minecraft.server.packs.PackResources);
    static {};
}
```
