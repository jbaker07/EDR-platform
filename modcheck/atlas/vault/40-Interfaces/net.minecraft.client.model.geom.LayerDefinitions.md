---
type: "interface"
fqcn: "net.minecraft.client.model.geom.LayerDefinitions"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.model.geom.LayerDefinitions

System: [[20-Systems/net.minecraft.client.model|net.minecraft.client.model]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `createRoots` | `@Inject at INVOKE Lcom/google/common/collect/ImmutableMap$Builder;build()Lcom/go` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.model.geom.LayerDefinitions {
    private static final net.minecraft.client.model.geom.builders.CubeDeformation FISH_PATTERN_DEFORMATION;
    private static final net.minecraft.client.model.geom.builders.CubeDeformation OUTER_ARMOR_DEFORMATION;
    private static final net.minecraft.client.model.geom.builders.CubeDeformation INNER_ARMOR_DEFORMATION;
    private static final net.minecraft.client.model.geom.builders.CubeDeformation BABY_OUTER_ARMOR_DEFORMATION;
    private static final net.minecraft.client.model.geom.builders.CubeDeformation BABY_INNER_ARMOR_DEFORMATION;
    private static final net.minecraft.client.model.geom.builders.CubeDeformation BABY_PIGLIN_INNER_ARMOR_DEFORMATION;
    private static final net.minecraft.client.model.geom.builders.CubeDeformation BABY_PIGLIN_OUTER_ARMOR_DEFORMATION;
    private static final net.minecraft.client.model.geom.PartPose BABY_PIGLIN_ARMOR_ARM_OFFSET;
    public net.minecraft.client.model.geom.LayerDefinitions();
    public static java.util.Map<net.minecraft.client.model.geom.ModelLayerLocation, net.minecraft.client.model.geom.builders.LayerDefinition> createRoots();
    private static boolean lambda$createRoots$10(com.google.common.collect.ImmutableMap, net.minecraft.client.model.geom.ModelLayerLocation);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$9(net.minecraft.client.model.geom.builders.MeshDefinition);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$8(net.minecraft.client.model.geom.builders.MeshTransformer, net.minecraft.client.model.geom.builders.LayerDefinition);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$7(net.minecraft.client.model.geom.builders.MeshTransformer, net.minecraft.client.model.geom.builders.LayerDefinition);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$6(net.minecraft.client.model.geom.builders.MeshTransformer, net.minecraft.client.model.geom.builders.LayerDefinition);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$5(net.minecraft.client.model.geom.builders.LayerDefinition);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$4(net.minecraft.client.model.geom.builders.MeshDefinition);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$3(net.minecraft.client.model.geom.builders.MeshDefinition);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$2(net.minecraft.client.model.geom.builders.MeshDefinition);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$1(net.minecraft.client.model.geom.builders.MeshDefinition);
    private static net.minecraft.client.model.geom.builders.LayerDefinition lambda$createRoots$0(net.minecraft.client.model.geom.builders.MeshDefinition);
    static {};
}
```
