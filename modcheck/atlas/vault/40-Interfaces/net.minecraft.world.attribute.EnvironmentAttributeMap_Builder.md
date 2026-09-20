---
type: "interface"
fqcn: "net.minecraft.world.attribute.EnvironmentAttributeMap$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.attribute.EnvironmentAttributeMap$Builder

System: [[20-Systems/net.minecraft.world.attribute|net.minecraft.world.attribute]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `build()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `putAll(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Lne` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.attribute.EnvironmentAttributeMap$Builder {
    private final java.util.Map<net.minecraft.world.attribute.EnvironmentAttribute<?>, net.minecraft.world.attribute.EnvironmentAttributeMap$Entry<?, ?>> entries;
    private net.minecraft.world.attribute.EnvironmentAttributeMap$Builder();
    public net.minecraft.world.attribute.EnvironmentAttributeMap$Builder putAll(net.minecraft.world.attribute.EnvironmentAttributeMap);
    public <Value, Parameter> net.minecraft.world.attribute.EnvironmentAttributeMap$Builder modify(net.minecraft.world.attribute.EnvironmentAttribute<Value>, net.minecraft.world.attribute.modifier.AttributeModifier<Value, Parameter>, Parameter);
    public <Value> net.minecraft.world.attribute.EnvironmentAttributeMap$Builder set(net.minecraft.world.attribute.EnvironmentAttribute<Value>, Value);
    public net.minecraft.world.attribute.EnvironmentAttributeMap build();
}
```
