---
type: "interface"
fqcn: "net.minecraft.CrashReport"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.CrashReport

System: [[20-Systems/net.minecraft|net.minecraft]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addCategory` | `(Ljava/lang/String;)Lnet/minecraft/CrashReportCategory;` | exact | invokevirtual@78 in `FluidStorageUtil.interactWithFluidStorage` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `addCategory` | `(Ljava/lang/String;)Lnet/minecraft/CrashReportCategory;` | exact | invokevirtual@305 in `StorageUtil.move` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `addCategory` | `(Ljava/lang/String;)Lnet/minecraft/CrashReportCategory;` | exact | invokevirtual@107 in `StorageUtil.extractAny` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `addCategory` | `(Ljava/lang/String;)Lnet/minecraft/CrashReportCategory;` | exact | invokevirtual@159 in `StorageUtil.insertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `addCategory` | `(Ljava/lang/String;)Lnet/minecraft/CrashReportCategory;` | exact | invokevirtual@64 in `StorageUtil.tryInsertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `forThrowable` | `(Ljava/lang/Throwable;Ljava/lang/String;)Lnet/minecraft/CrashReport;` | exact | invokestatic@69 in `FluidStorageUtil.interactWithFluidStorage` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `forThrowable` | `(Ljava/lang/Throwable;Ljava/lang/String;)Lnet/minecraft/CrashReport;` | exact | invokestatic@296 in `StorageUtil.move` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `forThrowable` | `(Ljava/lang/Throwable;Ljava/lang/String;)Lnet/minecraft/CrashReport;` | exact | invokestatic@98 in `StorageUtil.extractAny` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `forThrowable` | `(Ljava/lang/Throwable;Ljava/lang/String;)Lnet/minecraft/CrashReport;` | exact | invokestatic@150 in `StorageUtil.insertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `forThrowable` | `(Ljava/lang/Throwable;Ljava/lang/String;)Lnet/minecraft/CrashReport;` | exact | invokestatic@55 in `StorageUtil.tryInsertStacking` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (9 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DATE_TIME_FORMATTER : Ljava/time/format/DateTimeFormatter;
private final title : Ljava/lang/String;
private final exception : Ljava/lang/Throwable;
private final details : Ljava/util/List;
private saveFile : Ljava/nio/file/Path;
private trackingStackTrace : Z
private uncategorizedStackTrace : [Ljava/lang/StackTraceElement;
private final systemReport : Lnet/minecraft/SystemReport;
public <init>(Ljava/lang/String;Ljava/lang/Throwable;)V
public getTitle()Ljava/lang/String;
public getException()Ljava/lang/Throwable;
public getDetails()Ljava/lang/String;
public getDetails(Ljava/lang/StringBuilder;)V
public getExceptionMessage()Ljava/lang/String;
private static copyProperties(Ljava/lang/Throwable;Ljava/lang/Throwable;)Ljava/lang/Throwable;
private static replaceMessage(Ljava/lang/Throwable;Ljava/lang/String;)Ljava/lang/Throwable;
public getFriendlyReport(Lnet/minecraft/ReportType;Ljava/util/List;)Ljava/lang/String;
public getFriendlyReport(Lnet/minecraft/ReportType;)Ljava/lang/String;
public getSaveFile()Ljava/nio/file/Path;
public saveToFile(Ljava/nio/file/Path;Lnet/minecraft/ReportType;Ljava/util/List;)Z
public saveToFile(Ljava/nio/file/Path;Lnet/minecraft/ReportType;)Z
public getSystemReport()Lnet/minecraft/SystemReport;
public addCategory(Ljava/lang/String;)Lnet/minecraft/CrashReportCategory;
public addCategory(Ljava/lang/String;I)Lnet/minecraft/CrashReportCategory;
public static forThrowable(Ljava/lang/Throwable;Ljava/lang/String;)Lnet/minecraft/CrashReport;
public static preload()V
static <clinit>()V
```
