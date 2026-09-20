---
type: "interface"
fqcn: "net.minecraft.client.resources.model.SimpleModelWrapper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.SimpleModelWrapper

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `findNonBlockSprites` | `@Inject at INVOKE Lnet/minecraft/client/resources/model/geometry/QuadCollection;` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.resources.model.SimpleModelWrapper extends java.lang.Record implements net.minecraft.client.renderer.block.dispatch.BlockStateModelPart {
    private final net.minecraft.client.resources.model.geometry.QuadCollection quads;
    private final boolean useAmbientOcclusion;
    private final net.minecraft.client.resources.model.sprite.Material$Baked particleMaterial;
    private static final org.slf4j.Logger LOGGER;
    public net.minecraft.client.resources.model.SimpleModelWrapper(net.minecraft.client.resources.model.geometry.QuadCollection, boolean, net.minecraft.client.resources.model.sprite.Material$Baked);
    public static net.minecraft.client.renderer.block.dispatch.BlockStateModelPart bake(net.minecraft.client.resources.model.ModelBaker, net.minecraft.resources.Identifier, net.minecraft.client.renderer.block.dispatch.ModelState);
    public static com.google.common.collect.Multimap<net.minecraft.resources.Identifier, net.minecraft.resources.Identifier> findNonBlockSprites(net.minecraft.client.resources.model.geometry.QuadCollection);
    public java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad> getQuads(net.minecraft.core.Direction);
    public int materialFlags();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.client.resources.model.geometry.QuadCollection quads();
    public boolean useAmbientOcclusion();
    public net.minecraft.client.resources.model.sprite.Material$Baked particleMaterial();
    static {};
}
```
