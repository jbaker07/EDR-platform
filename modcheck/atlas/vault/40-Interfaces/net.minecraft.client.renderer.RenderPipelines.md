---
type: "interface"
fqcn: "net.minecraft.client.renderer.RenderPipelines"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.RenderPipelines

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `GUI_TEXTURED` | `Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;` | exact | getstatic@77 in `FabricCreativeGuiComponents$CreativeModeTabButton.extractContents` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `GUI_TEXTURED` | `Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;` | exact | getstatic@10 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GUI_TEXTURED` | `Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;` | exact | getstatic@49 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GUI_TEXTURED` | `Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;` | exact | getstatic@79 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (184 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final PIPELINES_BY_LOCATION : Ljava/util/Map;
private static final OPTIONAL_PIPELINES_BY_LOCATION : Ljava/util/Map;
private static final ALPHA_CUTOUT_THRESHOLD_DEFAULT : F
private static final ALPHA_CUTOUT_THRESHOLD_CUTOUT_TERRAIN : F
public static final GLOBALS_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final MATRICES_FOG_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final MATRICES_FOG_LIGHT_DIR_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_DEPTH_BOUNDS_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_TRANSMITTANCE_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_ACCUMULATE_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final GENERIC_BLOCKS_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final LIT_BLOCKS_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final TERRAIN_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final MULTIDRAW_TERRAIN_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final BLOCK_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final WATER_MASK_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final LIGHTNING_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final DRAGON_RAYS_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final ENTITY_NO_LIGHTMAP_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final ENTITY_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_ENTITY_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final ENTITY_EMISSIVE_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final EYES_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final BEACON_BEAM_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final ITEM_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_ITEM_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final TEXT_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final WORLD_TEXT_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final END_PORTAL_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final CLOUDS_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_CLOUDS_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final LINES_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_LINES_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final DEBUG_FILLED_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_DEBUG_FILLED_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final DEBUG_POINTS_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final PARTICLE_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OIT_PARTICLE_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final WEATHER_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final CRUMBLING_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final GUI_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final GUI_TEXTURED_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final GUI_TEXT_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final OUTLINE_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final POST_PROCESSING_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final SOLID_BLOCK : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final SOLID_TERRAIN : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final SOLID_TERRAIN_MULTIDRAW : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final WIREFRAME : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final WIREFRAME_MULTIDRAW : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final CUTOUT_BLOCK : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final CUTOUT_TERRAIN : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final CUTOUT_TERRAIN_MULTIDRAW : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final TRANSLUCENT_TERRAIN : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final TRANSLUCENT_TERRAIN_MULTIDRAW : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_TERRAIN : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final OIT_TERRAIN_MULTIDRAW : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final TRANSLUCENT_BLOCK : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_TRANSLUCENT_BLOCK : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final ARMOR_CUTOUT_NO_CULL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ARMOR_DECAL_CUTOUT_NO_CULL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final WOLF_ARMOR_CRACKS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ENTITY_SOLID : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ENTITY_SOLID_Z_OFFSET_FORWARD : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ENTITY_CUTOUT_CULL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ENTITY_CUTOUT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ENTITY_CUTOUT_Z_OFFSET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ENTITY_CUTOUT_DISSOLVE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ENTITY_TRANSLUCENT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_ENTITY : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final ENTITY_TRANSLUCENT_EMISSIVE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_ENTITY_EMISSIVE : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final ENTITY_TRANSLUCENT_CULL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_ENTITY_CULL : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final END_CRYSTAL_BEAM : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final BANNER_PATTERN : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final BREEZE_WIND : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_BREEZE_WIND : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final ENERGY_SWIRL_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final ENERGY_SWIRL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_ENERGY_SWIRL : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final EYES : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_EYES : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final ENTITY_SHADOW_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final ENTITY_SHADOW : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_ENTITY_SHADOW : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final GLINT_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final GLINT_SPECIAL_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final ARMOR_CUTOUT_NO_CULL_GLINT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ENTITY_SOLID_GLINT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ITEM_CUTOUT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ITEM_CUTOUT_GLINT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ITEM_CUTOUT_GLINT_SPECIAL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ITEM_TRANSLUCENT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_ITEM : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final ITEM_TRANSLUCENT_GLINT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ITEM_TRANSLUCENT_GLINT_SPECIAL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_ITEM_GLINT : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final OIT_ITEM_GLINT_SPECIAL : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final BEACON_BEAM_OPAQUE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final BEACON_BEAM_TRANSLUCENT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_BEACON_BEAM : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final LEASH : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final WATER_MASK : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_WATER_MASK : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final GLINT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final CRUMBLING : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_CRUMBLING : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final TEXT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_TEXT : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final GUI_TEXT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final TEXT_GRAYSCALE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_TEXT_GRAYSCALE : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final GUI_TEXT_GRAYSCALE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final TEXT_POLYGON_OFFSET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_TEXT_POLYGON_OFFSET : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final TEXT_GRAYSCALE_POLYGON_OFFSET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_TEXT_GRAYSCALE_POLYGON_OFFSET : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final TEXT_SEE_THROUGH : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final TEXT_GRAYSCALE_SEE_THROUGH : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final LIGHTNING : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_LIGHTNING : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final DRAGON_RAYS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_DRAGON_RAYS : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final END_PORTAL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final END_GATEWAY : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final FLAT_CLOUDS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final CLOUDS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_CLOUDS : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final OIT_FLAT_CLOUDS : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final LINES : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final LINES_TRANSLUCENT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final LINES_TRANSLUCENT_NO_DEPTH_WRITE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_LINES_TRANSLUCENT : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final LINES_DEPTH_BIAS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final SECONDARY_BLOCK_OUTLINE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final DEBUG_POINTS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_DEBUG_POINTS : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final DEBUG_FILLED_BOX : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_DEBUG_FILLED_BOX : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final DEBUG_QUADS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_DEBUG_QUADS : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final DEBUG_TRIANGLE_FAN : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_DEBUG_TRIANGLE_FAN : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final WORLD_BORDER : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_WORLD_BORDER : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final OPAQUE_PARTICLE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final TRANSLUCENT_PARTICLE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_PARTICLE : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final WEATHER : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_WEATHER : Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static final SKY : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final END_SKY : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final SUNRISE_SUNSET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final STARS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final CELESTIAL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final GUI : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final GUI_INVERT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final GUI_TEXT_HIGHLIGHT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final GUI_TEXTURED : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final GUI_TEXTURED_PREMULTIPLIED_ALPHA : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final BLOCK_SCREEN_EFFECT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final FIRE_SCREEN_EFFECT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final GUI_OPAQUE_TEXTURED_BACKGROUND : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final GUI_NAUSEA_OVERLAY : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final VIGNETTE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final CROSSHAIR : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final MOJANG_LOGO : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ENTITY_OUTLINE_BLIT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final TRACY_BLIT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final PANORAMA : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OUTLINE_CULL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OUTLINE_NO_CULL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final LIGHTMAP : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ANIMATE_SPRITE_SNIPPET : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Snippet;
public static final ANIMATE_SPRITE_BLIT : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final ANIMATE_SPRITE_INTERPOLATE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final BLIT_DEPTH_BOUNDS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_DEPTH_BOUNDS_CULL : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final BLIT_DEPTH_DURING_DEPTH_BOUNDS : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final BLIT_DEPTH : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final OIT_COMPOSITE : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static final INTEGRATE_DEPTH : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public <init>()V
public static register(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static register(Lnet/minecraft/client/renderer/oit/OitPipelineSet;)Lnet/minecraft/client/renderer/oit/OitPipelineSet;
public static registerOptional(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;)Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
public static requiredPipelines()Ljava/util/List;
public static optionalPipelines()Ljava/util/List;
private static synthetic lambda$static$11(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$10(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$9(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$8(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$7(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$6(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$5(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$4(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$3(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$2(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$1(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline$Builder;)V
private static synthetic lambda$static$0()Lcom/mojang/renderpearl/api/pipeline/ColorTargetState;
static <clinit>()V
```
