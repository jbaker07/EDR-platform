---
type: "interface"
fqcn: "net.minecraft.core.Direction"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Direction

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`enum` public final; extends `java/lang/Enum`; implements `net/minecraft/core/Directional`, `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@10 in `ModelHelper.toFaceIndex` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@25 in `QuadSpriteBaker.bakeSprite` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@49 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@64 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@79 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@94 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@109 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@124 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@13 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@17 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `get3DDataValue` | `()I` | exact | invokevirtual@4 in `AoFace.get` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getApproximateNearest` | `(FFF)Lnet/minecraft/core/Direction;` | exact | invokestatic@46 in `ItemSheetedDecalTextureGenerator.setNormal` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getAxis` | `()Lnet/minecraft/core/Direction$Axis;` | exact | invokevirtual@1 in `GeometryHelper.isQuadParallelToFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getAxis` | `()Lnet/minecraft/core/Direction$Axis;` | exact | invokevirtual@3 in `GeometryHelper.isParallelQuadOnFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getAxis` | `()Lnet/minecraft/core/Direction$Axis;` | exact | invokevirtual@5 in `ComposterWrapper.get` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getAxisDirection` | `()Lnet/minecraft/core/Direction$AxisDirection;` | exact | invokevirtual@16 in `GeometryHelper.isParallelQuadOnFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getOpposite` | `()Lnet/minecraft/core/Direction;` | exact | invokevirtual@25 in `FlowingFluidMixin.shouldSpreadLiquid` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getOpposite` | `()Lnet/minecraft/core/Direction;` | exact | invokevirtual@13 in `LavaFluidMixin.shouldSpreadLiquid` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getOpposite` | `()Lnet/minecraft/core/Direction;` | exact | invokevirtual@27 in `CrafterBlockMixin.transferOrSpawnStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOpposite` | `()Lnet/minecraft/core/Direction;` | exact | invokevirtual@38 in `DropperBlockMixin.hookDispense` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOpposite` | `()Lnet/minecraft/core/Direction;` | exact | invokevirtual@31 in `HopperBlockEntityMixin.hookInsert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getRotation` | `()Lorg/joml/Quaternionf;` | exact | invokevirtual@98 in `ItemSheetedDecalTextureGenerator.setNormal` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getUnitVec3i` | `()Lnet/minecraft/core/Vec3i;` | exact | invokevirtual@20 in `NormalHelper.computeFaceNormal` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@35 in `WorldlyContainerSlotWrapper.toString` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@45 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@60 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@75 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@90 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@45 in `MutableQuadView.square` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@69 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@84 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@99 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@114 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@129 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@144 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@4 in `GeometryHelper.isQuadCubic` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@33 in `GeometryHelper.firstCubicVertex` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@8 in `AltModelBlockRendererImpl.shouldCullFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `rotate` | `(Lorg/joml/Matrix4fc;Lnet/minecraft/core/Direction;)Lnet/minecraft/cor` | exact | invokestatic@328 in `ModelStateHelper.lambda$asQuadTransform$1` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `toString` | `()Ljava/lang/String;` | exact | invokevirtual@232 in `AoCalculator.compute` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/core/Direction;` | exact | invokestatic@0 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/core/Direction;` | exact | invokestatic@0 in `ModelHelper.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/core/Direction;` | exact | invokestatic@0 in `ModelStateHelper.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/core/Direction;` | exact | invokestatic@54 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/core/Direction;` | exact | invokestatic@81 in `EncodingFormat.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/core/Direction;` | exact | invokestatic@0 in `ExtendedBlockModelFeatureRenderer.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@32 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@32 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@27 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@46 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@245 in `AoCalculator.irregularFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@139 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@205 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@259 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@313 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@102 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@111 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@82 in `GeometryHelper.lightFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWN` | `Lnet/minecraft/core/Direction;` | exact | getstatic@36 in `HopperBlockEntityMixin.hookExtract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@42 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@121 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@100 in `AoCalculator.irregularFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@19 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@73 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@145 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@199 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@16 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@66 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `EAST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@55 in `GeometryHelper.lightFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@87 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@76 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@399 in `AoCalculator.irregularFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@25 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@85 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@265 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@325 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@158 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@141 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NORTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@103 in `GeometryHelper.lightFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@72 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@91 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@393 in `AoCalculator.irregularFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@31 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@91 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@271 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@331 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@128 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@126 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SOUTH` | `Lnet/minecraft/core/Direction;` | exact | getstatic@97 in `GeometryHelper.lightFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@4 in `BlockTransformerHelperImpl.createTilling` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@4 in `BlockTransformerHelperImpl.createFlattening` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@12 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@61 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@239 in `AoCalculator.irregularFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@133 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@211 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@253 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@319 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@72 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@96 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@76 in `GeometryHelper.lightFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@107 in `GeometryHelper.lightFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@47 in `ComposterWrapper.get` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `UP` | `Lnet/minecraft/core/Direction;` | exact | getstatic@57 in `HopperBlockEntityMixin.hookExtract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@57 in `MutableQuadView$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@106 in `QuadSpriteBaker.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@106 in `AoCalculator.irregularFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@13 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@79 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@151 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@193 in `AoFace.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@46 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@81 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `WEST` | `Lnet/minecraft/core/Direction;` | exact | getstatic@61 in `GeometryHelper.lightFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (27 fields, 61 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DOWN : Lnet/minecraft/core/Direction;
public static final UP : Lnet/minecraft/core/Direction;
public static final NORTH : Lnet/minecraft/core/Direction;
public static final SOUTH : Lnet/minecraft/core/Direction;
public static final WEST : Lnet/minecraft/core/Direction;
public static final EAST : Lnet/minecraft/core/Direction;
public static final CODEC : Lnet/minecraft/util/StringRepresentable$EnumCodec;
public static final VERTICAL_CODEC : Lcom/mojang/serialization/Codec;
public static final BY_ID : Ljava/util/function/IntFunction;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final LEGACY_ID_CODEC : Lcom/mojang/serialization/Codec;
public static final LEGACY_ID_CODEC_2D : Lcom/mojang/serialization/Codec;
private static final YXZ_AXIS_ORDER : Lcom/google/common/collect/ImmutableList;
private static final YZX_AXIS_ORDER : Lcom/google/common/collect/ImmutableList;
private final data3d : I
private final oppositeIndex : I
private final data2d : I
private final name : Ljava/lang/String;
private final axis : Lnet/minecraft/core/Direction$Axis;
private final axisDirection : Lnet/minecraft/core/Direction$AxisDirection;
private final normal : Lnet/minecraft/core/Vec3i;
private final normalVec3 : Lnet/minecraft/world/phys/Vec3;
private final normalVec3f : Lorg/joml/Vector3fc;
private static final VALUES : [Lnet/minecraft/core/Direction;
private static final BY_3D_DATA : [Lnet/minecraft/core/Direction;
private static final BY_2D_DATA : [Lnet/minecraft/core/Direction;
private static final synthetic $VALUES : [Lnet/minecraft/core/Direction;
public static values()[Lnet/minecraft/core/Direction;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/core/Direction;
private <init>(Ljava/lang/String;IIIILjava/lang/String;Lnet/minecraft/core/Direction$AxisDirection;Lnet/minecraft/core/Direction$Axis;Lnet/minecraft/core/Vec3i;)V
public static orderedByNearest(Lnet/minecraft/world/entity/Entity;)[Lnet/minecraft/core/Direction;
private static makeDirectionArray(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;)[Lnet/minecraft/core/Direction;
public static rotate(Lorg/joml/Matrix4fc;Lnet/minecraft/core/Direction;)Lnet/minecraft/core/Direction;
public static allShuffled(Lnet/minecraft/util/RandomSource;)Ljava/util/Collection;
public static stream()Ljava/util/stream/Stream;
public static getYRot(Lnet/minecraft/core/Direction;)F
public getRotation()Lorg/joml/Quaternionf;
public get3DDataValue()I
public get2DDataValue()I
public getAxisDirection()Lnet/minecraft/core/Direction$AxisDirection;
public static getFacingAxis(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Direction$Axis;)Lnet/minecraft/core/Direction;
public getOpposite()Lnet/minecraft/core/Direction;
public getClockWise(Lnet/minecraft/core/Direction$Axis;)Lnet/minecraft/core/Direction;
public getCounterClockWise(Lnet/minecraft/core/Direction$Axis;)Lnet/minecraft/core/Direction;
public getClockWise()Lnet/minecraft/core/Direction;
private getClockWiseX()Lnet/minecraft/core/Direction;
private getCounterClockWiseX()Lnet/minecraft/core/Direction;
private getClockWiseZ()Lnet/minecraft/core/Direction;
private getCounterClockWiseZ()Lnet/minecraft/core/Direction;
public getCounterClockWise()Lnet/minecraft/core/Direction;
public getStepX()I
public getStepY()I
public getStepZ()I
public getStep()Lnet/minecraft/core/Vec3i;
public step()Lorg/joml/Vector3f;
public getName()Ljava/lang/String;
public getAxis()Lnet/minecraft/core/Direction$Axis;
public static byName(Ljava/lang/String;)Lnet/minecraft/core/Direction;
public static from3DDataValue(I)Lnet/minecraft/core/Direction;
public static from2DDataValue(I)Lnet/minecraft/core/Direction;
public static fromYRot(D)Lnet/minecraft/core/Direction;
public static fromAxisAndDirection(Lnet/minecraft/core/Direction$Axis;Lnet/minecraft/core/Direction$AxisDirection;)Lnet/minecraft/core/Direction;
public toYRot()F
public static getRandom(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/core/Direction;
public static getApproximateNearest(DDD)Lnet/minecraft/core/Direction;
public static getApproximateNearest(FFF)Lnet/minecraft/core/Direction;
public static getApproximateNearest(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/core/Direction;
public static getNearest(IIILnet/minecraft/core/Direction;)Lnet/minecraft/core/Direction;
public static getNearest(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecraft/core/Direction;
public toString()Ljava/lang/String;
public getSerializedName()Ljava/lang/String;
private static verifyVertical(Lnet/minecraft/core/Direction;)Lcom/mojang/serialization/DataResult;
public static get(Lnet/minecraft/core/Direction$AxisDirection;Lnet/minecraft/core/Direction$Axis;)Lnet/minecraft/core/Direction;
public static axisStepOrder(Lnet/minecraft/world/phys/Vec3;)Lcom/google/common/collect/ImmutableList;
public getUnitVec3i()Lnet/minecraft/core/Vec3i;
public getUnitVec3()Lnet/minecraft/world/phys/Vec3;
public getUnitVec3f()Lorg/joml/Vector3fc;
public isFacingAngle(F)Z
private static synthetic $values()[Lnet/minecraft/core/Direction;
private static synthetic lambda$verifyVertical$0()Ljava/lang/String;
private static synthetic lambda$BY_2D_DATA$0(I)[Lnet/minecraft/core/Direction;
private static synthetic lambda$static$4(Lnet/minecraft/core/Direction;)I
private static synthetic lambda$static$3(Lnet/minecraft/core/Direction;)Z
private static synthetic lambda$BY_3D_DATA$0(I)[Lnet/minecraft/core/Direction;
private static synthetic lambda$static$2(Lnet/minecraft/core/Direction;)I
private static synthetic lambda$static$1(Lnet/minecraft/core/Direction;)Ljava/lang/Byte;
private static synthetic lambda$static$0(Lnet/minecraft/core/Direction;)Ljava/lang/Byte;
static <clinit>()V
```
