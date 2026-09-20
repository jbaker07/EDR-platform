---
type: "interface"
fqcn: "net.minecraft.CrashReportCategory"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.CrashReportCategory

System: [[20-Systems/net.minecraft|net.minecraft]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `setDetail` | `(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCatego` | exact | invokevirtual@95 in `FluidStorageUtil.interactWithFluidStorage` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCatego` | exact | invokevirtual@362 in `StorageUtil.move` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCatego` | exact | invokevirtual@369 in `StorageUtil.move` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCatego` | exact | invokevirtual@132 in `StorageUtil.extractAny` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCatego` | exact | invokevirtual@138 in `StorageUtil.extractAny` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCatego` | exact | invokevirtual@190 in `StorageUtil.insertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCatego` | exact | invokevirtual@197 in `StorageUtil.insertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCatego` | exact | invokevirtual@95 in `StorageUtil.tryInsertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCatego` | exact | invokevirtual@102 in `StorageUtil.tryInsertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@89 in `FluidStorageUtil.interactWithFluidStorage` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@112 in `FluidStorageUtil.interactWithFluidStorage` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@123 in `FluidStorageUtil.interactWithFluidStorage` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@321 in `StorageUtil.move` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@337 in `StorageUtil.move` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@353 in `StorageUtil.move` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@123 in `StorageUtil.extractAny` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@170 in `StorageUtil.insertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@181 in `StorageUtil.insertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@75 in `StorageUtil.tryInsertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/Cr` | exact | invokevirtual@86 in `StorageUtil.tryInsertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (3 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final title : Ljava/lang/String;
private final entries : Ljava/util/List;
private stackTrace : [Ljava/lang/StackTraceElement;
public <init>(Ljava/lang/String;)V
public static formatLocation(Lnet/minecraft/world/level/LevelHeightAccessor;DDD)Ljava/lang/String;
public static formatLocation(Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/core/BlockPos;)Ljava/lang/String;
public static formatLocation(Lnet/minecraft/world/level/LevelHeightAccessor;III)Ljava/lang/String;
public setDetail(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)Lnet/minecraft/CrashReportCategory;
public setDetail(Ljava/lang/String;Ljava/lang/Object;)Lnet/minecraft/CrashReportCategory;
public setDetailError(Ljava/lang/String;Ljava/lang/Throwable;)V
public fillInStackTrace(I)I
public validateStackTrace(Ljava/lang/StackTraceElement;Ljava/lang/StackTraceElement;)Z
public getDetails(Ljava/lang/StringBuilder;)V
public getStacktrace()[Ljava/lang/StackTraceElement;
public static populateBlockDetails(Lnet/minecraft/CrashReportCategory;Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public static populateBlockLocationDetails(Lnet/minecraft/CrashReportCategory;Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/CrashReportCategory;
private static synthetic lambda$populateBlockLocationDetails$0(Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/core/BlockPos;)Ljava/lang/String;
```
