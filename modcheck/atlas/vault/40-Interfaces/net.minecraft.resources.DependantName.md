---
type: "interface"
fqcn: "net.minecraft.resources.DependantName"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.DependantName

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `fixed(Ljava/lang/Object;)Lnet/minecraft/resources/DependantName;` | `` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (3, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.resources.DependantName<T, V> {
    public abstract V get(net.minecraft.resources.ResourceKey<T>);
    public static <T, V> net.minecraft.resources.DependantName<T, V> fixed(V);
    private static java.lang.Object lambda$fixed$0(java.lang.Object, net.minecraft.resources.ResourceKey);
}
```
