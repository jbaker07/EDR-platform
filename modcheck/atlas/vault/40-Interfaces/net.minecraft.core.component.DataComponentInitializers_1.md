---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentInitializers$1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentInitializers$1

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/core/component/DataComponentInitializers$PendingComponents`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| injects_into | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Ljava/util/List;)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `apply` | `()V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic val$registryKey : Lnet/minecraft/resources/ResourceKey;
final synthetic val$entries : Ljava/util/List;
 <init>(Lnet/minecraft/resources/ResourceKey;Ljava/util/List;)V
public key()Lnet/minecraft/resources/ResourceKey;
public forEach(Ljava/util/function/BiConsumer;)V
public apply()V
private static synthetic lambda$forEach$0(Ljava/util/function/BiConsumer;Lnet/minecraft/core/component/DataComponentInitializers$BakedEntry;)V
```
