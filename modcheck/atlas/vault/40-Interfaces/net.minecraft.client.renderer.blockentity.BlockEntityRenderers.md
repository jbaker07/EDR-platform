---
type: "interface"
fqcn: "net.minecraft.client.renderer.blockentity.BlockEntityRenderers"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.blockentity.BlockEntityRenderers

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<clinit>*` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.blockentity.BlockEntityRenderers {
    private static final java.util.Map<net.minecraft.world.level.block.entity.BlockEntityType<?>, net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider<?, ?>> PROVIDERS;
    public net.minecraft.client.renderer.blockentity.BlockEntityRenderers();
    private static <T extends net.minecraft.world.level.block.entity.BlockEntity, S extends net.minecraft.client.renderer.blockentity.state.BlockEntityRenderState> void register(net.minecraft.world.level.block.entity.BlockEntityType<? extends T>, net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider<T, S>);
    public static java.util.Map<net.minecraft.world.level.block.entity.BlockEntityType<?>, net.minecraft.client.renderer.blockentity.BlockEntityRenderer<?, ?>> createEntityRenderers(net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider$Context);
    private static void lambda$createEntityRenderers$0(com.google.common.collect.ImmutableMap$Builder, net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider$Context, net.minecraft.world.level.block.entity.BlockEntityType, net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider);
    private static net.minecraft.client.renderer.blockentity.BlockEntityRenderer lambda$static$5(net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider$Context);
    private static net.minecraft.client.renderer.blockentity.BlockEntityRenderer lambda$static$4(net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider$Context);
    private static net.minecraft.client.renderer.blockentity.BlockEntityRenderer lambda$static$3(net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider$Context);
    private static net.minecraft.client.renderer.blockentity.BlockEntityRenderer lambda$static$2(net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider$Context);
    private static net.minecraft.client.renderer.blockentity.BlockEntityRenderer lambda$static$1(net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider$Context);
    private static net.minecraft.client.renderer.blockentity.BlockEntityRenderer lambda$static$0(net.minecraft.client.renderer.blockentity.BlockEntityRendererProvider$Context);
    static {};
}
```
