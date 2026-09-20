---
type: "interface"
fqcn: "net.minecraft.core.particles.ParticleType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.particles.ParticleType

System: [[20-Systems/net.minecraft.core.particles|net.minecraft.core.particles]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Z)V` | exact | invokespecial@12 in `FabricParticleTypes$2.<init>` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `<init>` | `(Z)V` | exact | invokespecial@12 in `FabricParticleTypes$3.<init>` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (1 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final overrideLimiter : Z
protected <init>(Z)V
public getOverrideLimiter()Z
public abstract codec()Lcom/mojang/serialization/MapCodec;
public abstract streamCodec()Lnet/minecraft/network/codec/StreamCodec;
```
