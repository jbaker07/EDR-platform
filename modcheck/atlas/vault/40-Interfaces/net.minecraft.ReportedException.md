---
type: "interface"
fqcn: "net.minecraft.ReportedException"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.ReportedException

System: [[20-Systems/net.minecraft|net.minecraft]]

`class` public; extends `java/lang/RuntimeException`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/CrashReport;)V` | exact | invokespecial@133 in `FluidStorageUtil.interactWithFluidStorage` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/CrashReport;)V` | exact | invokespecial@379 in `StorageUtil.move` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/CrashReport;)V` | exact | invokespecial@148 in `StorageUtil.extractAny` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/CrashReport;)V` | exact | invokespecial@207 in `StorageUtil.insertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/CrashReport;)V` | exact | invokespecial@112 in `StorageUtil.tryInsertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (1 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final report : Lnet/minecraft/CrashReport;
public <init>(Lnet/minecraft/CrashReport;)V
public getReport()Lnet/minecraft/CrashReport;
public getCause()Ljava/lang/Throwable;
public getMessage()Ljava/lang/String;
```
