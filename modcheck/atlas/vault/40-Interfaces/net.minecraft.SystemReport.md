---
type: "interface"
fqcn: "net.minecraft.SystemReport"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.SystemReport

System: [[20-Systems/net.minecraft|net.minecraft]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `setDetail` | `(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-crash-report-info-v1|fabric-crash-report-info-v1]] | declared |
| injects_into | `<init>` | `()V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-crash-report-info-v1|fabric-crash-report-info-v1]] | direct_reference |

## Declared members (7 fields, 58 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final BYTES_PER_MEBIBYTE : J
private static final ONE_GIGA : J
private static final LOGGER : Lorg/slf4j/Logger;
private static final OPERATING_SYSTEM : Ljava/lang/String;
private static final JAVA_VERSION : Ljava/lang/String;
private static final JAVA_VM_VERSION : Ljava/lang/String;
private final entries : Ljava/util/List;
public <init>()V
private static printMemoryUsage(Ljava/lang/management/MemoryUsage;)Ljava/lang/String;
private static printJvmFlags(Ljava/util/function/Predicate;)Ljava/lang/String;
public setDetail(Ljava/lang/String;Ljava/lang/String;)V
public setDetail(Ljava/lang/String;Lnet/minecraft/CrashReportDetail;)V
private putHardware(Loshi/SystemInfo;)V
private putSoftware(Loshi/SystemInfo;)V
private ignoreErrors(Ljava/lang/String;Ljava/lang/Runnable;)V
public static sizeInMiB(J)F
private putPhysicalMemory(Ljava/util/List;)V
private putVirtualMemory(Loshi/hardware/VirtualMemory;)V
private putMemory(Loshi/hardware/GlobalMemory;)V
private putGraphics(Ljava/util/List;)V
private putProcessor(Loshi/hardware/CentralProcessor;)V
private putStorage()V
private putProcessDetails(Loshi/software/os/OSProcess;)V
private putSpaceForProperty(Ljava/lang/String;)V
private putSpaceForPath(Ljava/lang/String;Ljava/util/function/Supplier;)V
public appendToCrashReportString(Ljava/lang/StringBuilder;)V
public toLineSeparatedString()Ljava/lang/String;
private static synthetic lambda$toLineSeparatedString$0(Lnet/minecraft/CrashReportCategory$Entry;)Ljava/lang/String;
private static synthetic lambda$appendToCrashReportString$0(Ljava/lang/StringBuilder;Lnet/minecraft/CrashReportCategory$Entry;)V
private static synthetic lambda$putSpaceForPath$0(Ljava/nio/file/FileStore;)Ljava/lang/Object;
private static synthetic lambda$putSpaceForProperty$0(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$putProcessDetails$2(Loshi/software/os/OSProcess;)Ljava/lang/Object;
private static synthetic lambda$putProcessDetails$1(Loshi/software/os/OSProcess;)Ljava/lang/Object;
private static synthetic lambda$putProcessDetails$0(Loshi/software/os/OSProcess;)Ljava/lang/Object;
private static synthetic lambda$putStorage$0()Ljava/lang/String;
private static synthetic lambda$putProcessor$3(Loshi/hardware/CentralProcessor;)Ljava/lang/Object;
private static synthetic lambda$putProcessor$2(Loshi/hardware/CentralProcessor;)Ljava/lang/Object;
private static synthetic lambda$putProcessor$1(Loshi/hardware/CentralProcessor;)Ljava/lang/Object;
private static synthetic lambda$putProcessor$0(Loshi/hardware/CentralProcessor$ProcessorIdentifier;)Ljava/lang/Object;
private static synthetic lambda$putGraphics$0(Loshi/hardware/GraphicsCard;)Ljava/lang/Object;
private synthetic lambda$putMemory$1(Loshi/hardware/GlobalMemory;)V
private synthetic lambda$putMemory$0(Loshi/hardware/GlobalMemory;)V
private static synthetic lambda$putVirtualMemory$3(Loshi/hardware/VirtualMemory;)Ljava/lang/Object;
private static synthetic lambda$putVirtualMemory$2(Loshi/hardware/VirtualMemory;)Ljava/lang/Object;
private static synthetic lambda$putVirtualMemory$1(Loshi/hardware/VirtualMemory;)Ljava/lang/Object;
private static synthetic lambda$putVirtualMemory$0(Loshi/hardware/VirtualMemory;)Ljava/lang/Object;
private static synthetic lambda$putPhysicalMemory$1(Loshi/hardware/PhysicalMemory;)Ljava/lang/Object;
private static synthetic lambda$putPhysicalMemory$0(Loshi/hardware/PhysicalMemory;)Ljava/lang/Object;
private synthetic lambda$putSoftware$0(Loshi/software/os/OperatingSystem;)V
private synthetic lambda$putHardware$2(Loshi/hardware/HardwareAbstractionLayer;)V
private synthetic lambda$putHardware$1(Loshi/hardware/HardwareAbstractionLayer;)V
private synthetic lambda$putHardware$0(Loshi/hardware/HardwareAbstractionLayer;)V
private static synthetic lambda$new$10()Ljava/lang/Object;
private static synthetic lambda$new$11(Ljava/lang/String;)Z
private static synthetic lambda$new$8()Ljava/lang/Object;
private static synthetic lambda$new$9(Ljava/lang/String;)Z
private synthetic lambda$new$7()V
private synthetic lambda$new$6()V
private static synthetic lambda$new$5()Ljava/lang/Object;
private static synthetic lambda$new$4()Ljava/lang/Object;
private static synthetic lambda$new$3()Ljava/lang/Object;
private static synthetic lambda$new$2()Ljava/lang/Object;
private static synthetic lambda$new$1()Ljava/lang/Object;
private static synthetic lambda$new$0()Ljava/lang/Object;
static <clinit>()V
```
