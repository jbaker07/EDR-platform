---
type: "interface"
fqcn: "net.minecraft.client.renderer.blockentity.BlockEntityRenderers"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.blockentity.BlockEntityRenderers

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<clinit>` | `?` | selector_unsupported | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `PROVIDERS` | `Ljava/util/Map;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |

## Declared members (1 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final PROVIDERS : Ljava/util/Map;
public <init>()V
public static register(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider;)V
public static createEntityRenderers(Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context;)Ljava/util/Map;
private static synthetic lambda$createEntityRenderers$0(Lcom/google/common/collect/ImmutableMap$Builder;Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context;Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider;)V
private static synthetic lambda$static$5(Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context;)Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderer;
private static synthetic lambda$static$4(Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context;)Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderer;
private static synthetic lambda$static$3(Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context;)Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderer;
private static synthetic lambda$static$2(Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context;)Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderer;
private static synthetic lambda$static$1(Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context;)Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderer;
private static synthetic lambda$static$0(Lnet/minecraft/client/renderer/blockentity/BlockEntityRendererProvider$Context;)Lnet/minecraft/client/renderer/blockentity/BlockEntityRenderer;
static <clinit>()V
```
