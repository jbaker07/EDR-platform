---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `submit(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase implements net.minecraft.client.renderer.feature.phase.FeatureRenderPhase<net.minecraft.client.renderer.feature.submit.SubmitNode> {
    private net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase$FeatureSubmits<?>[] submitsByFeature;
    public net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase();
    public void submit(net.minecraft.client.renderer.feature.submit.SubmitNode);
    public void sortInto(net.minecraft.client.renderer.feature.phase.FeatureRenderPhase$Output);
    private static <Submit extends net.minecraft.client.renderer.feature.submit.SubmitNode> void sortFeatureInto(net.minecraft.client.renderer.feature.phase.FeatureRenderPhase$Output, net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase$FeatureSubmits<Submit>);
    private static <V> java.util.Collection<V> maybeShuffle(java.util.Collection<V>);
    private static <V> V[] maybeShuffle(V[]);
    public void clear();
    public boolean isEmpty();
}
```
