---
type: "interface"
fqcn: "net.minecraft.util.RandomSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.RandomSource

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createThreadLocalInstance` | `(J)Lnet/minecraft/util/RandomSource;` | exact | invokestatic@6 in `LevelExtractorMixin.<init>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `createThreadLocalInstance` | `(J)Lnet/minecraft/util/RandomSource;` | exact | invokestatic@19 in `SubmitNodeCollectionMixin.hasMaterialFlagProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `createThreadLocalInstance` | `(J)Lnet/minecraft/util/RandomSource;` | exact | invokestatic@16 in `AltModelBlockRendererImpl.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `nextLong` | `()J` | exact | invokeinterface@1 in `CompositeBlockStateModelImpl.collectParts` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `nextLong` | `()J` | exact | invokeinterface@2 in `CompositeBlockStateModelImpl.emitQuads` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `nextLong` | `()J` | exact | invokeinterface@9 in `CompositeBlockStateModelImpl.createGeometryKey` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `nextLong` | `()J` | exact | invokeinterface@24 in `MultiPartModelMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `nextLong` | `()J` | exact | invokeinterface@35 in `MultiPartModelMixin.createGeometryKey` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `nextLong` | `()J` | exact | invokeinterface@24 in `MultiPartModelMixin.materialFlags` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@37 in `CompositeBlockStateModelImpl.collectParts` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@41 in `CompositeBlockStateModelImpl.emitQuads` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@26 in `CompositeBlockStateModelImpl.createGeometryKey` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@91 in `CompositeBlockStateModelImpl.createGeometryKey` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@68 in `MultiPartModelMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@52 in `MultiPartModelMixin.createGeometryKey` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@105 in `MultiPartModelMixin.createGeometryKey` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@71 in `MultiPartModelMixin.materialFlags` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@10 in `LevelExtractorMixin.hasMaterialFlagProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setSeed` | `(J)V` | exact | invokeinterface@113 in `AltModelBlockRendererImpl.tesselateBlock` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (1 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final GAUSSIAN_SPREAD_FACTOR : D
public static create()Lnet/minecraft/util/RandomSource;
public static createThreadSafe()Lnet/minecraft/util/RandomSource;
public static create(J)Lnet/minecraft/util/RandomSource;
public static createThreadLocalInstance()Lnet/minecraft/util/RandomSource;
public static createThreadLocalInstance(J)Lnet/minecraft/util/RandomSource;
public abstract fork()Lnet/minecraft/util/RandomSource;
public abstract forkPositional()Lnet/minecraft/world/level/levelgen/PositionalRandomFactory;
public abstract setSeed(J)V
public abstract nextInt()I
public abstract nextInt(I)I
public nextIntBetweenInclusive(II)I
public abstract nextLong()J
public abstract nextBoolean()Z
public abstract nextFloat()F
public abstract nextDouble()D
public abstract nextGaussian()D
public triangle(DD)D
public triangle(FF)F
public consumeCount(I)V
public nextInt(II)I
```
