---
type: "interface"
fqcn: "net.minecraft.client.resources.model.sprite.MaterialBaker"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.sprite.MaterialBaker

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `spriteFinder(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadAtlas;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.resources.model.sprite.MaterialBaker {
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.client.renderer.texture.SpriteLoader$Preparations blockAtlas;
    private final net.minecraft.client.renderer.texture.SpriteLoader$Preparations itemAtlas;
    private final net.minecraft.client.resources.model.sprite.Material$Baked missingSprite;
    private final net.minecraft.client.resources.model.sprite.Material$Baked missingSpriteForceTranslucent;
    private final com.google.common.collect.Multimap<java.lang.String, net.minecraft.resources.Identifier> missingSprites;
    private final com.google.common.collect.Multimap<java.lang.String, java.lang.String> missingReferences;
    private final java.util.Map<net.minecraft.client.resources.model.sprite.Material, net.minecraft.client.resources.model.sprite.Material$Baked> bakedMaterials;
    private final java.util.function.Function<net.minecraft.client.resources.model.sprite.Material, net.minecraft.client.resources.model.sprite.Material$Baked> bakerFunction;
    public net.minecraft.client.resources.model.sprite.MaterialBaker(net.minecraft.client.renderer.texture.SpriteLoader$Preparations, net.minecraft.client.renderer.texture.SpriteLoader$Preparations);
    private net.minecraft.client.resources.model.sprite.Material$Baked replacementForMissingMaterial(net.minecraft.client.resources.model.sprite.Material);
    public net.minecraft.client.resources.model.sprite.Material$Baked get(net.minecraft.client.resources.model.sprite.Material, net.minecraft.client.resources.model.ModelDebugName);
    private net.minecraft.client.resources.model.sprite.Material$Baked bake(net.minecraft.client.resources.model.sprite.Material);
    private static net.minecraft.client.resources.model.sprite.Material$Baked bakeForAtlas(net.minecraft.client.resources.model.sprite.Material, net.minecraft.client.renderer.texture.SpriteLoader$Preparations);
    public net.minecraft.client.resources.model.sprite.Material$Baked resolveSlot(net.minecraft.client.resources.model.sprite.TextureSlots, java.lang.String, net.minecraft.client.resources.model.ModelDebugName);
    public net.minecraft.client.resources.model.sprite.Material$Baked reportMissingReference(java.lang.String, net.minecraft.client.resources.model.ModelDebugName);
    public void logMissingTextures();
    private static void lambda$logMissingTextures$2(java.lang.String, java.util.Collection);
    private static java.lang.String lambda$logMissingTextures$3(java.lang.String);
    private static void lambda$logMissingTextures$0(java.lang.String, java.util.Collection);
    private static java.lang.String lambda$logMissingTextures$1(net.minecraft.resources.Identifier);
    static {};
}
```
