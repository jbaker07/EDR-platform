---
type: "interface"
fqcn: "net.minecraft.client.resources.language.ClientLanguage"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.language.ClientLanguage

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`class` public; extends `net/minecraft/locale/Language`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `loadFrom` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final storage : Ljava/util/Map;
private final defaultRightToLeft : Z
private <init>(Ljava/util/Map;Z)V
public static loadFrom(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Z)Lnet/minecraft/client/resources/language/ClientLanguage;
private static appendFrom(Ljava/lang/String;Ljava/util/List;Ljava/util/Map;)V
public getOrDefault(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
public has(Ljava/lang/String;)Z
public isDefaultRightToLeft()Z
public getVisualOrder(Lnet/minecraft/network/chat/FormattedText;)Lnet/minecraft/util/FormattedCharSequence;
static <clinit>()V
```
