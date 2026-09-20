---
type: "interface"
fqcn: "net.minecraft.resources.RegistryValidator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryValidator

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `none()Lnet/minecraft/resources/RegistryValidator;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.resources.RegistryValidator<T> {
    public static final net.minecraft.resources.RegistryValidator<?> NONE;
    public static final net.minecraft.resources.RegistryValidator<?> NON_EMPTY;
    public static <T> net.minecraft.resources.RegistryValidator<T> none();
    public static <T> net.minecraft.resources.RegistryValidator<T> nonEmpty();
    public abstract void validate(net.minecraft.core.Registry<T>, java.util.Map<net.minecraft.resources.ResourceKey<?>, java.lang.Exception>);
    private static void lambda$static$1(net.minecraft.core.Registry, java.util.Map);
    private static void lambda$static$0(net.minecraft.core.Registry, java.util.Map);
    static {};
}
```
