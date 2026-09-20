---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.ParticlesRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.ParticlesRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `reset` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.state.level.ParticlesRenderState {
    public final java.util.List<net.minecraft.client.renderer.state.level.ParticleGroupRenderState> particles;
    public net.minecraft.client.renderer.state.level.ParticlesRenderState();
    public void reset();
    public void add(net.minecraft.client.renderer.state.level.ParticleGroupRenderState);
    public void submit(net.minecraft.client.renderer.SubmitNodeCollector, net.minecraft.client.renderer.state.level.CameraRenderState);
}
```
