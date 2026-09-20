---
type: "interface"
fqcn: "net.minecraft.nbt.NbtAccounter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.NbtAccounter

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `unlimitedHeap` | `()Lnet/minecraft/nbt/NbtAccounter;` | exact | invokestatic@16 in `RegistryCustomContentState.readFile` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `unlimitedHeap` | `()Lnet/minecraft/nbt/NbtAccounter;` | exact | invokestatic@139 in `DefaultResourcePackStorage.read` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (7 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DEFAULT_NBT_QUOTA : I
public static final UNCOMPRESSED_NBT_QUOTA : I
private static final MAX_STACK_DEPTH : I
private final quota : J
private usage : J
private final maxDepth : I
private depth : I
public <init>(JI)V
public static create(J)Lnet/minecraft/nbt/NbtAccounter;
public static defaultQuota()Lnet/minecraft/nbt/NbtAccounter;
public static uncompressedQuota()Lnet/minecraft/nbt/NbtAccounter;
public static unlimitedHeap()Lnet/minecraft/nbt/NbtAccounter;
public accountBytes(JJ)V
public accountBytes(J)V
public pushDepth()V
public popDepth()V
public getUsage()J
public getDepth()I
```
