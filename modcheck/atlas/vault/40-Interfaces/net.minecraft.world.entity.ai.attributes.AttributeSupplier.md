---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.attributes.AttributeSupplier"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.attributes.AttributeSupplier

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `builder()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplie` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.entity.ai.attributes.AttributeSupplier {
    private final java.util.Map<net.minecraft.core.Holder<net.minecraft.world.entity.ai.attributes.Attribute>, net.minecraft.world.entity.ai.attributes.AttributeInstance> instances;
    private net.minecraft.world.entity.ai.attributes.AttributeSupplier(java.util.Map<net.minecraft.core.Holder<net.minecraft.world.entity.ai.attributes.Attribute>, net.minecraft.world.entity.ai.attributes.AttributeInstance>);
    private net.minecraft.world.entity.ai.attributes.AttributeInstance getAttributeInstance(net.minecraft.core.Holder<net.minecraft.world.entity.ai.attributes.Attribute>);
    public double getValue(net.minecraft.core.Holder<net.minecraft.world.entity.ai.attributes.Attribute>);
    public double getBaseValue(net.minecraft.core.Holder<net.minecraft.world.entity.ai.attributes.Attribute>);
    public double getModifierValue(net.minecraft.core.Holder<net.minecraft.world.entity.ai.attributes.Attribute>, net.minecraft.resources.Identifier);
    public net.minecraft.world.entity.ai.attributes.AttributeInstance createInstance(java.util.function.Consumer<net.minecraft.world.entity.ai.attributes.AttributeInstance>, net.minecraft.core.Holder<net.minecraft.world.entity.ai.attributes.Attribute>);
    public static net.minecraft.world.entity.ai.attributes.AttributeSupplier$Builder builder();
    public boolean hasAttribute(net.minecraft.core.Holder<net.minecraft.world.entity.ai.attributes.Attribute>);
    public boolean hasModifier(net.minecraft.core.Holder<net.minecraft.world.entity.ai.attributes.Attribute>, net.minecraft.resources.Identifier);
}
```
