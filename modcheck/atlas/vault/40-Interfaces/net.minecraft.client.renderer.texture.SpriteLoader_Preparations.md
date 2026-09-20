---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.SpriteLoader$Preparations"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.SpriteLoader$Preparations

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements `net/fabricmc/fabric/api/client/renderer/v1/sprite/FabricPreparations`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `missing` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@67 in `MaterialBakerMixin.spriteFinder` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `spriteFinder` | `()Lnet/fabricmc/fabric/api/client/renderer/v1/sprite/SpriteFinder;` | inherited_exact | invokevirtual@14 in `MaterialBakerMixin.spriteFinder` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `spriteFinder` | `()Lnet/fabricmc/fabric/api/client/renderer/v1/sprite/SpriteFinder;` | inherited_exact | invokevirtual@32 in `MaterialBakerMixin.spriteFinder` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `missing` | `Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `regions` | `Ljava/util/Map;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |

## Declared members (6 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final width : I
private final height : I
private final mipLevel : I
private final missing : Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
private final regions : Ljava/util/Map;
private final readyForUpload : Ljava/util/concurrent/CompletableFuture;
public <init>(IIILnet/minecraft/client/renderer/texture/TextureAtlasSprite;Ljava/util/Map;Ljava/util/concurrent/CompletableFuture;)V
public getSprite(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public width()I
public height()I
public mipLevel()I
public missing()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public regions()Ljava/util/Map;
public readyForUpload()Ljava/util/concurrent/CompletableFuture;
```
