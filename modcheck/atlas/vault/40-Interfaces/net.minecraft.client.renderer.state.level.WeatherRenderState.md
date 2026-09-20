---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.WeatherRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.WeatherRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `reset` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.state.level.WeatherRenderState {
    public final java.util.List<net.minecraft.client.renderer.WeatherEffectRenderer$ColumnInstance> rainColumns;
    public final java.util.List<net.minecraft.client.renderer.WeatherEffectRenderer$ColumnInstance> snowColumns;
    public float intensity;
    public int radius;
    public net.minecraft.client.renderer.state.level.WeatherRenderState();
    public void reset();
}
```
