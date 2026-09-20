---
type: "interface"
fqcn: "net.minecraft.server.network.ConfigurationTask$Type"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ConfigurationTask$Type

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;)V` | exact | invokespecial@10 in `AttachmentSync$AttachmentSyncTask.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/String;)V` | exact | invokespecial@13 in `CommonPacketsImpl$CommonRegisterConfigurationTask.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/String;)V` | exact | invokespecial@13 in `CommonPacketsImpl$CommonVersionConfigurationTask.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/String;)V` | exact | invokespecial@10 in `CustomIngredientSync$IngredientSyncTask.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/String;)V` | exact | invokespecial@6 in `RegistrySyncManager$SyncConfigurationTask.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | exact | invokevirtual@36 in `ServerConfigurationPacketListenerImplMixin.completeTask` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Ljava/lang/String;` | exact | invokevirtual@30 in `ServerConfigurationPacketListenerImplMixin.onClientReady` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Ljava/lang/String;` | exact | invokevirtual@37 in `ServerConfigurationPacketListenerImplMixin.pollEarlyTasks` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Ljava/lang/String;
public <init>(Ljava/lang/String;)V
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public id()Ljava/lang/String;
```
