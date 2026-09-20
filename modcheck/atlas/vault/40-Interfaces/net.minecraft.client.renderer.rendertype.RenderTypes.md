---
type: "interface"
fqcn: "net.minecraft.client.renderer.rendertype.RenderTypes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.rendertype.RenderTypes

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `cutoutMovingBlock` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@52 in `ChunkSectionLayerHelper.getMovingBlockRenderType` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `solidMovingBlock` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@46 in `ChunkSectionLayerHelper.getMovingBlockRenderType` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucentMovingBlock` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokestatic@58 in `ChunkSectionLayerHelper.getMovingBlockRenderType` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (57 fields, 101 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
static final OUTLINE : Ljava/util/function/BiFunction;
private static final SOLID_MOVING_BLOCK : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final CUTOUT_MOVING_BLOCK : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final TRANSLUCENT_MOVING_BLOCK : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final ARMOR_CUTOUT_NO_CULL : Ljava/util/function/Function;
private static final ARMOR_CUTOUT_NO_CULL_GLINT : Ljava/util/function/Function;
private static final ARMOR_TRIM : Ljava/util/function/Function;
private static final ARMOR_TRIM_DECAL : Ljava/util/function/Function;
private static final WOLF_ARMOR_CRACKS : Ljava/util/function/Function;
private static final ENTITY_SOLID : Ljava/util/function/Function;
private static final ENTITY_SOLID_GLINT : Ljava/util/function/Function;
private static final ENTITY_SOLID_Z_OFFSET_FORWARD : Ljava/util/function/Function;
private static final ENTITY_CUTOUT_CULL : Ljava/util/function/Function;
private static final ENTITY_CUTOUT : Ljava/util/function/BiFunction;
private static final ENTITY_CUTOUT_Z_OFFSET : Ljava/util/function/BiFunction;
private static final ENTITY_CUTOUT_DISSOLVE : Ljava/util/function/BiFunction;
private static final ENTITY_TRANSLUCENT_CULL : Ljava/util/function/Function;
private static final ITEM_CUTOUT : Ljava/util/function/Function;
private static final ITEM_CUTOUT_GLINT : Ljava/util/function/Function;
private static final ITEM_CUTOUT_GLINT_SPECIAL : Ljava/util/function/Function;
private static final ITEM_TRANSLUCENT : Ljava/util/function/Function;
private static final ITEM_TRANSLUCENT_GLINT : Ljava/util/function/Function;
private static final ITEM_TRANSLUCENT_GLINT_SPECIAL : Ljava/util/function/Function;
private static final ENTITY_TRANSLUCENT : Ljava/util/function/BiFunction;
private static final ENTITY_TRANSLUCENT_EMISSIVE : Ljava/util/function/BiFunction;
private static final END_CRYSTAL_BEAM : Ljava/util/function/Function;
private static final BEACON_BEAM : Ljava/util/function/BiFunction;
private static final BANNER_PATTERN : Ljava/util/function/Function;
private static final ENTITY_SHADOW : Ljava/util/function/Function;
private static final EYES : Ljava/util/function/Function;
private static final LEASH : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final WATER_MASK : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final OIT_WATER_MASK : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final PATTERNED_SHIELD_GLINT : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final TRIMMED_ARMOR_GLINT : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final CRUMBLING : Ljava/util/function/Function;
private static final TEXT : Ljava/util/function/Function;
private static final TEXT_GRAYSCALE : Ljava/util/function/Function;
private static final TEXT_POLYGON_OFFSET : Ljava/util/function/Function;
private static final TEXT_GRAYSCALE_POLYGON_OFFSET : Ljava/util/function/Function;
private static final TEXT_SEE_THROUGH : Ljava/util/function/Function;
private static final TEXT_GRAYSCALE_SEE_THROUGH : Ljava/util/function/Function;
private static final LIGHTNING : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final DRAGON_RAYS : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final END_PORTAL : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final END_GATEWAY : Lnet/minecraft/client/renderer/rendertype/RenderType;
public static final LINES : Lnet/minecraft/client/renderer/rendertype/RenderType;
public static final LINES_TRANSLUCENT : Lnet/minecraft/client/renderer/rendertype/RenderType;
public static final LINES_TRANSLUCENT_NO_DEPTH_WRITE : Lnet/minecraft/client/renderer/rendertype/RenderType;
public static final LINES_DEPTH_BIAS : Lnet/minecraft/client/renderer/rendertype/RenderType;
public static final SECONDARY_BLOCK_OUTLINE : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final DEBUG_FILLED_BOX : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final DEBUG_POINT : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final DEBUG_QUADS : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final DEBUG_TRIANGLE_FAN : Lnet/minecraft/client/renderer/rendertype/RenderType;
private static final BLOCK_SCREEN_EFFECT : Ljava/util/function/Function;
private static final FIRE_SCREEN_EFFECT : Ljava/util/function/Function;
public <init>()V
private static createMovingBlockSetup(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Z)Lnet/minecraft/client/renderer/rendertype/RenderSetup;
public static solidMovingBlock()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static cutoutMovingBlock()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static translucentMovingBlock()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static armorCutoutNoCull(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static armorCutoutNoCullGlint(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static armorTrim(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static wolfArmorCracks(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entitySolid(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entitySolidGlint(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entitySolidZOffsetForward(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityCutoutCull(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityCutout(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityCutout(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityCutoutZOffset(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityCutoutZOffset(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityCutoutDissolve(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityTranslucentCull(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static itemCutout(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static itemCutoutGlint(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static itemCutoutGlintSpecial(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static itemTranslucent(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static itemTranslucentGlint(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static itemTranslucentGlintSpecial(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityTranslucent(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityTranslucent(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityTranslucentEmissive(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static endCrystalBeam(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static beaconBeam(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static bannerPattern(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static entityShadow(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static eyes(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static breezeEyes(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static breezeWind(Lnet/minecraft/resources/Identifier;FF)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static energySwirl(Lnet/minecraft/resources/Identifier;FF)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static leash()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static waterMask()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static outline(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static patternedShieldGlint()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static trimmedArmorGlint()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static crumbling(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static text(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static textGrayscale(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static textPolygonOffset(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static textGrayscalePolygonOffset(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static textSeeThrough(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static textGrayscaleSeeThrough(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static lightning()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static dragonRays()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static endPortal()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static endGateway()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static lines()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static linesTranslucent()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static linesTranslucentNoDepthWrite()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static linesDepthBias()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static secondaryBlockOutline()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static debugFilledBox()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static debugPoint()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static debugQuads()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static debugTriangleFan()Lnet/minecraft/client/renderer/rendertype/RenderType;
public static blockScreenEffect(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static fireScreenEffect(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$35(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$34(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$33(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$32(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$31(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$30(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$29(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$28(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$27(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$26(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$25(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$24(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$23(Lnet/minecraft/resources/Identifier;Ljava/lang/Boolean;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$22(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$21(Lnet/minecraft/resources/Identifier;Ljava/lang/Boolean;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$20(Lnet/minecraft/resources/Identifier;Ljava/lang/Boolean;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$19(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$18(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$17(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$16(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$15(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$14(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$13(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$12(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$11(Lnet/minecraft/resources/Identifier;Ljava/lang/Boolean;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$10(Lnet/minecraft/resources/Identifier;Ljava/lang/Boolean;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$9(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$8(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$7(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$6(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$5(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$4(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$3(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$2(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$static$1(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$createMovingBlockSetup$0()Lcom/mojang/renderpearl/api/textures/GpuSampler;
private static synthetic lambda$static$0(Lnet/minecraft/resources/Identifier;Ljava/lang/Boolean;)Lnet/minecraft/client/renderer/rendertype/RenderType;
static <clinit>()V
```
