---
type: "interface"
fqcn: "net.minecraft.core.HolderSet$Named"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderSet$Named

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `net/minecraft/core/HolderSet$ListBacked`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `key` | `()Lnet/minecraft/tags/TagKey;` | exact | invokevirtual@1 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarnin | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/tags/TagKey;` | exact | invokevirtual@21 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarni | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/tags/TagKey;` | exact | invokevirtual@35 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarni | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `contents` | `Ljava/util/List;` | exact | getfield@118 in `MappedRegistryMixin.fabric_applyPendingTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| writes | `contents` | `Ljava/util/List;` | exact | putfield@279 in `MappedRegistryMixin.fabric_applyPendingTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (3 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final owner : Lnet/minecraft/core/HolderOwner;
private final key : Lnet/minecraft/tags/TagKey;
private contents : Ljava/util/List;
 <init>(Lnet/minecraft/core/HolderOwner;Lnet/minecraft/tags/TagKey;)V
 bind(Ljava/util/List;)V
public key()Lnet/minecraft/tags/TagKey;
protected contents()Ljava/util/List;
public isBound()Z
public unwrap()Lcom/mojang/datafixers/util/Either;
public unwrapKey()Ljava/util/Optional;
public contains(Lnet/minecraft/core/Holder;)Z
public toString()Ljava/lang/String;
public canSerializeIn(Lnet/minecraft/core/HolderOwner;)Z
```
