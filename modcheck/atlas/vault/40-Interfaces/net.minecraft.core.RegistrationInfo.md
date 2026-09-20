---
type: "interface"
fqcn: "net.minecraft.core.RegistrationInfo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistrationInfo

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/Optional;Lcom/mojang/serialization/Lifecycle;)V` | exact | invokespecial@334 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/util/Optional;Lcom/mojang/serialization/Lifecycle;)V` | exact | invokespecial@162 in `DimensionModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/util/Optional;Lcom/mojang/serialization/Lifecycle;)V` | exact | invokespecial@105 in `ResourceManagerRegistryLoadTaskMixin.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `knownPackInfo` | `()Ljava/util/Optional;` | exact | invokevirtual@55 in `WorldDimensionsMixin.betterModdedStabilityCheck` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `lifecycle` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokevirtual@331 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lifecycle` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokevirtual@159 in `DimensionModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `lifecycle` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokevirtual@73 in `WorldDimensionsMixin.betterModdedStabilityCheck` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `lifecycle` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokevirtual@102 in `ResourceManagerRegistryLoadTaskMixin.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `BUILT_IN` | `Lnet/minecraft/core/RegistrationInfo;` | exact | getstatic@59 in `FabricRegistryBuilder.buildAndRegister` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (3 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final knownPackInfo : Ljava/util/Optional;
private final lifecycle : Lcom/mojang/serialization/Lifecycle;
public static final BUILT_IN : Lnet/minecraft/core/RegistrationInfo;
public <init>(Ljava/util/Optional;Lcom/mojang/serialization/Lifecycle;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public knownPackInfo()Ljava/util/Optional;
public lifecycle()Lcom/mojang/serialization/Lifecycle;
static <clinit>()V
```
