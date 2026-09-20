---
type: "interface"
fqcn: "net.minecraft.util.StrictJsonParser"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.StrictJsonParser

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `parse` | `(Ljava/io/Reader;)Lcom/google/gson/JsonElement;` | exact | invokestatic@6 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `parse` | `(Ljava/io/Reader;)Lcom/google/gson/JsonElement;` | exact | invokestatic@6 in `FabricLanguageProvider$TranslationBuilder.overwriteWith` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `parse` | `(Ljava/io/Reader;)Lcom/google/gson/JsonElement;` | exact | invokestatic@145 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `parse` | `(Ljava/io/Reader;)Lcom/google/gson/JsonElement;` | exact | invokestatic@65 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public static parse(Ljava/io/Reader;)Lcom/google/gson/JsonElement;
public static parse(Ljava/lang/String;)Lcom/google/gson/JsonElement;
```
