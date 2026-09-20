---
type: "interface"
fqcn: "net.minecraft.core.particles.SimpleParticleType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.particles.SimpleParticleType

System: [[20-Systems/net.minecraft.core.particles|net.minecraft.core.particles]]

`class` public; extends `net/minecraft/core/particles/ParticleType`; implements `net/minecraft/core/particles/ParticleOptions`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Z)V` | exact | invokespecial@2 in `FabricParticleTypes$1.<init>` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final codec : Lcom/mojang/serialization/MapCodec;
private final streamCodec : Lnet/minecraft/network/codec/StreamCodec;
protected <init>(Z)V
public getType()Lnet/minecraft/core/particles/SimpleParticleType;
public codec()Lcom/mojang/serialization/MapCodec;
public streamCodec()Lnet/minecraft/network/codec/StreamCodec;
public synthetic getType()Lnet/minecraft/core/particles/ParticleType;
```
