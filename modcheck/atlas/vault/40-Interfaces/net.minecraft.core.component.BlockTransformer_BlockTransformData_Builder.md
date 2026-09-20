---
type: "interface"
fqcn: "net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.BlockTransformer$BlockTransformData$Builder

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `build` | `()Lnet/minecraft/core/component/BlockTransformer$BlockTransformData;` | exact | invokevirtual@30 in `BlockTransformerHelperImpl.createStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/BlockTransformer$BlockTransformData;` | exact | invokevirtual@41 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/BlockTransformer$BlockTransformData;` | exact | invokevirtual@41 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/BlockTransformer$BlockTransformData;` | exact | invokevirtual@36 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/core/component/BlockTransformer$BlockTransformData;` | exact | invokevirtual@36 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `disallowedFaces` | `(Ljava/util/List;)Lnet/minecraft/core/component/BlockTransformer$Block` | exact | invokevirtual@38 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `disallowedFaces` | `(Ljava/util/List;)Lnet/minecraft/core/component/BlockTransformer$Block` | exact | invokevirtual@38 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `particle` | `(Lnet/minecraft/core/component/BlockTransformer$TransformParticle;)Lne` | exact | invokevirtual@33 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `particle` | `(Lnet/minecraft/core/component/BlockTransformer$TransformParticle;)Lne` | exact | invokevirtual@33 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `sound` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/core/component/BlockTransf` | exact | invokevirtual@27 in `BlockTransformerHelperImpl.createStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `sound` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/core/component/BlockTransf` | exact | invokevirtual@29 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `sound` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/core/component/BlockTransf` | exact | invokevirtual@29 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `sound` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/core/component/BlockTransf` | exact | invokevirtual@27 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `sound` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/core/component/BlockTransf` | exact | invokevirtual@27 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (10 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final targetStateProvider : Lnet/minecraft/core/Holder;
private sound : Lnet/minecraft/core/Holder;
private particle : Lnet/minecraft/core/component/BlockTransformer$TransformParticle;
private disallowedFaces : Ljava/util/List;
private loot : Ljava/util/Optional;
private dropStrategy : Lnet/minecraft/core/component/BlockTransformer$DropStrategy;
private updateFromNeighbors : Z
private transformType : Lnet/minecraft/core/component/BlockTransformer$TransformType;
private consumeOnUse : Z
private itemDamagePerUse : I
private <init>(Lnet/minecraft/core/Holder;)V
public sound(Lnet/minecraft/core/Holder;)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public particle(Lnet/minecraft/core/component/BlockTransformer$TransformParticle;)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public disallowedFaces(Ljava/util/List;)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public loot(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public dropStrategy(Lnet/minecraft/core/component/BlockTransformer$DropStrategy;)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public updateFromNeighbors(Z)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public transformType(Lnet/minecraft/core/component/BlockTransformer$TransformType;)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public consumeOnUse(Z)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public itemDamagePerUse(I)Lnet/minecraft/core/component/BlockTransformer$BlockTransformData$Builder;
public build()Lnet/minecraft/core/component/BlockTransformer$BlockTransformData;
```
