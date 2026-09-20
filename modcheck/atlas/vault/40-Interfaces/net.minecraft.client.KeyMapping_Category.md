---
type: "interface"
fqcn: "net.minecraft.client.KeyMapping$Category"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.KeyMapping$Category

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `CategoryComparator.compare` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@14 in `CategoryComparator.compare` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@52 in `CategoryComparator.compare` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@59 in `CategoryComparator.compare` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@79 in `CategoryComparator.compare` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@86 in `CategoryComparator.compare` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| injects_into | `register` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/KeyMapping` | exact | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| reads | `SORT_ORDER` | `Ljava/util/List;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | declared |

## Declared members (10 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Lnet/minecraft/resources/Identifier;
private static final SORT_ORDER : Ljava/util/List;
public static final MOVEMENT : Lnet/minecraft/client/KeyMapping$Category;
public static final MISC : Lnet/minecraft/client/KeyMapping$Category;
public static final MULTIPLAYER : Lnet/minecraft/client/KeyMapping$Category;
public static final GAMEPLAY : Lnet/minecraft/client/KeyMapping$Category;
public static final INVENTORY : Lnet/minecraft/client/KeyMapping$Category;
public static final CREATIVE : Lnet/minecraft/client/KeyMapping$Category;
public static final SPECTATOR : Lnet/minecraft/client/KeyMapping$Category;
public static final DEBUG : Lnet/minecraft/client/KeyMapping$Category;
public <init>(Lnet/minecraft/resources/Identifier;)V
private static register(Ljava/lang/String;)Lnet/minecraft/client/KeyMapping$Category;
public static register(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/KeyMapping$Category;
public label()Lnet/minecraft/network/chat/Component;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public id()Lnet/minecraft/resources/Identifier;
static <clinit>()V
```
