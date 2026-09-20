---
type: "interface"
fqcn: "net.minecraft.util.filefix.fixes.DimensionStorageFileFix"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.filefix.fixes.DimensionStorageFileFix

System: [[20-Systems/net.minecraft.util.filefix|net.minecraft.util.filefix]]

`class` public; extends `net/minecraft/util/filefix/FileFix`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `makeFixer` | `()V` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (0 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>(Lcom/mojang/datafixers/schemas/Schema;)V
public makeFixer()V
```
