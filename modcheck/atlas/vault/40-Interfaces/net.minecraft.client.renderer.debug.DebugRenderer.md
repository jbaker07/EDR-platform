---
type: "interface"
fqcn: "net.minecraft.client.renderer.debug.DebugRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.debug.DebugRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `refreshRendererList` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.debug.DebugRenderer {
    private final java.util.List<net.minecraft.client.renderer.debug.DebugRenderer$SimpleDebugRenderer> renderers;
    private long lastDebugEntriesVersion;
    public net.minecraft.client.renderer.debug.DebugRenderer();
    public void refreshRendererList();
    public void emitGizmos(net.minecraft.client.renderer.culling.Frustum, double, double, double, float);
    public static java.util.Optional<net.minecraft.world.entity.Entity> getTargetedEntity(net.minecraft.world.entity.Entity, int);
    private static net.minecraft.world.phys.Vec3 mixColor(float);
    private static net.minecraft.world.phys.Vec3 shiftHue(float, float, float, float);
}
```
