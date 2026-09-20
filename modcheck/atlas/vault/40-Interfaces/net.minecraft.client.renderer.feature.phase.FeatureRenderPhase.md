---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.phase.FeatureRenderPhase"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.phase.FeatureRenderPhase

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `submit(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (3, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.renderer.feature.phase.FeatureRenderPhase<Submit extends net.minecraft.client.renderer.feature.submit.SubmitNode> {
    public abstract void submit(Submit);
    public abstract void sortInto(net.minecraft.client.renderer.feature.phase.FeatureRenderPhase$Output);
    public abstract boolean isEmpty();
}
```
