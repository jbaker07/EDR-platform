---
type: "interface"
fqcn: "net.minecraft.server.network.config.SynchronizeRegistriesTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.config.SynchronizeRegistriesTask

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/network/ConfigurationTask`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `sendRegistries` | `(Ljava/util/function/Consumer;Ljava/util/Set;)V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| injects_into | `handleResponse` | `(Ljava/util/List;Ljava/util/function/Consumer;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `sendRegistries` | `(Ljava/util/function/Consumer;Ljava/util/Set;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `start` | `(Ljava/util/function/Consumer;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `requestedPacks` | `Ljava/util/List;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |

## Declared members (3 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final TYPE : Lnet/minecraft/server/network/ConfigurationTask$Type;
private final requestedPacks : Ljava/util/List;
private final registries : Lnet/minecraft/core/LayeredRegistryAccess;
public <init>(Ljava/util/List;Lnet/minecraft/core/LayeredRegistryAccess;)V
public start(Ljava/util/function/Consumer;)V
private sendRegistries(Ljava/util/function/Consumer;Ljava/util/Set;)V
public handleResponse(Ljava/util/List;Ljava/util/function/Consumer;)V
public type()Lnet/minecraft/server/network/ConfigurationTask$Type;
private static synthetic lambda$sendRegistries$0(Ljava/util/function/Consumer;Lnet/minecraft/resources/ResourceKey;Ljava/util/List;)V
static <clinit>()V
```
