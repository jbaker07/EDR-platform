---
type: "interface"
fqcn: "net.minecraft.client.particle.ParticleRenderType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.ParticleRenderType

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@29 in `ParticleGroupRegistry.getId` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@43 in `ParticleGroupRegistry.getId` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `ELDER_GUARDIANS` | `Lnet/minecraft/client/particle/ParticleRenderType;` | exact | getstatic@15 in `ParticleGroupRegistry.getId` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `ITEM_PICKUP` | `Lnet/minecraft/client/particle/ParticleRenderType;` | exact | getstatic@22 in `ParticleGroupRegistry.getId` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `NO_RENDER` | `Lnet/minecraft/client/particle/ParticleRenderType;` | exact | getstatic@8 in `ParticleGroupRegistry.getId` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `SINGLE_QUADS` | `Lnet/minecraft/client/particle/ParticleRenderType;` | exact | getstatic@1 in `ParticleGroupRegistry.getId` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (6 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final name : Ljava/lang/String;
private final shorthand : Ljava/lang/String;
public static final SINGLE_QUADS : Lnet/minecraft/client/particle/ParticleRenderType;
public static final ITEM_PICKUP : Lnet/minecraft/client/particle/ParticleRenderType;
public static final ELDER_GUARDIANS : Lnet/minecraft/client/particle/ParticleRenderType;
public static final NO_RENDER : Lnet/minecraft/client/particle/ParticleRenderType;
public <init>(Ljava/lang/String;Ljava/lang/String;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public name()Ljava/lang/String;
public shorthand()Ljava/lang/String;
static <clinit>()V
```
