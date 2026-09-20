---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.attributes.DefaultAttributes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.attributes.DefaultAttributes

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<clinit>*` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.entity.ai.attributes.DefaultAttributes {
    private static final org.slf4j.Logger LOGGER;
    private static final java.util.Map<net.minecraft.world.entity.EntityType<? extends net.minecraft.world.entity.LivingEntity>, net.minecraft.world.entity.ai.attributes.AttributeSupplier> SUPPLIERS;
    public net.minecraft.world.entity.ai.attributes.DefaultAttributes();
    public static net.minecraft.world.entity.ai.attributes.AttributeSupplier getSupplier(net.minecraft.world.entity.EntityType<? extends net.minecraft.world.entity.LivingEntity>);
    public static boolean hasSupplier(net.minecraft.world.entity.EntityType<?>);
    public static void validate();
    private static void lambda$validate$2(net.minecraft.resources.Identifier);
    private static boolean lambda$validate$1(net.minecraft.world.entity.EntityType);
    private static boolean lambda$validate$0(net.minecraft.world.entity.EntityType);
    static {};
}
```
