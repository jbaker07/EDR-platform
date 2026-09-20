---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.ArmorModelSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.ArmorModelSet

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `putFrom(Lnet/minecraft/client/renderer/entity/ArmorModelSet;Lcom/go` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.entity.ArmorModelSet<T> extends java.lang.Record {
    private final T head;
    private final T chest;
    private final T legs;
    private final T feet;
    public net.minecraft.client.renderer.entity.ArmorModelSet(T, T, T, T);
    public T get(net.minecraft.world.entity.EquipmentSlot);
    public <U> net.minecraft.client.renderer.entity.ArmorModelSet<U> map(java.util.function.Function<? super T, ? extends U>);
    public void putFrom(net.minecraft.client.renderer.entity.ArmorModelSet<net.minecraft.client.model.geom.builders.LayerDefinition>, com.google.common.collect.ImmutableMap$Builder<T, net.minecraft.client.model.geom.builders.LayerDefinition>);
    public static <M extends net.minecraft.client.model.HumanoidModel<?>> net.minecraft.client.renderer.entity.ArmorModelSet<M> bake(net.minecraft.client.renderer.entity.ArmorModelSet<net.minecraft.client.model.geom.ModelLayerLocation>, net.minecraft.client.model.geom.EntityModelSet, java.util.function.Function<net.minecraft.client.model.geom.ModelPart, M>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public T head();
    public T chest();
    public T legs();
    public T feet();
    private static net.minecraft.client.model.HumanoidModel lambda$bake$0(java.util.function.Function, net.minecraft.client.model.geom.EntityModelSet, net.minecraft.client.model.geom.ModelLayerLocation);
}
```
