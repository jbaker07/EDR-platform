---
type: "interface"
fqcn: "net.minecraft.SystemReport"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.SystemReport

System: [[20-Systems/net.minecraft|net.minecraft]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-crash-report-info-v1|fabric-crash-report-info-v1]] | direct_reference |

## Declared members (65, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.SystemReport {
    public static final long BYTES_PER_MEBIBYTE;
    private static final long ONE_GIGA;
    private static final org.slf4j.Logger LOGGER;
    private static final java.lang.String OPERATING_SYSTEM;
    private static final java.lang.String JAVA_VERSION;
    private static final java.lang.String JAVA_VM_VERSION;
    private final java.util.List<net.minecraft.CrashReportCategory$Entry> entries;
    public net.minecraft.SystemReport();
    private static java.lang.String printMemoryUsage(java.lang.management.MemoryUsage);
    private static java.lang.String printJvmFlags(java.util.function.Predicate<java.lang.String>);
    public void setDetail(java.lang.String, java.lang.String);
    public void setDetail(java.lang.String, net.minecraft.CrashReportDetail<java.lang.Object>);
    private void putHardware(oshi.SystemInfo);
    private void putSoftware(oshi.SystemInfo);
    private void ignoreErrors(java.lang.String, java.lang.Runnable);
    public static float sizeInMiB(long);
    private void putPhysicalMemory(java.util.List<oshi.hardware.PhysicalMemory>);
    private void putVirtualMemory(oshi.hardware.VirtualMemory);
    private void putMemory(oshi.hardware.GlobalMemory);
    private void putGraphics(java.util.List<oshi.hardware.GraphicsCard>);
    private void putProcessor(oshi.hardware.CentralProcessor);
    private void putStorage();
    private void putProcessDetails(oshi.software.os.OSProcess);
    private void putSpaceForProperty(java.lang.String);
    private void putSpaceForPath(java.lang.String, java.util.function.Supplier<java.lang.String>);
    public void appendToCrashReportString(java.lang.StringBuilder);
    public java.lang.String toLineSeparatedString();
    private static java.lang.String lambda$toLineSeparatedString$0(net.minecraft.CrashReportCategory$Entry);
    private static void lambda$appendToCrashReportString$0(java.lang.StringBuilder, net.minecraft.CrashReportCategory$Entry);
    private static java.lang.Object lambda$putSpaceForPath$0(java.nio.file.FileStore) throws java.lang.Exception;
    private static java.lang.String lambda$putSpaceForProperty$0(java.lang.String);
    private static java.lang.Object lambda$putProcessDetails$2(oshi.software.os.OSProcess) throws java.lang.Exception;
    private static java.lang.Object lambda$putProcessDetails$1(oshi.software.os.OSProcess) throws java.lang.Exception;
    private static java.lang.Object lambda$putProcessDetails$0(oshi.software.os.OSProcess) throws java.lang.Exception;
    private static java.lang.String lambda$putStorage$0();
    private static java.lang.Object lambda$putProcessor$3(oshi.hardware.CentralProcessor) throws java.lang.Exception;
    private static java.lang.Object lambda$putProcessor$2(oshi.hardware.CentralProcessor) throws java.lang.Exception;
    private static java.lang.Object lambda$putProcessor$1(oshi.hardware.CentralProcessor) throws java.lang.Exception;
    private static java.lang.Object lambda$putProcessor$0(oshi.hardware.CentralProcessor$ProcessorIdentifier) throws java.lang.Exception;
    private static java.lang.Object lambda$putGraphics$0(oshi.hardware.GraphicsCard) throws java.lang.Exception;
    private void lambda$putMemory$1(oshi.hardware.GlobalMemory);
    private void lambda$putMemory$0(oshi.hardware.GlobalMemory);
    private static java.lang.Object lambda$putVirtualMemory$3(oshi.hardware.VirtualMemory) throws java.lang.Exception;
    private static java.lang.Object lambda$putVirtualMemory$2(oshi.hardware.VirtualMemory) throws java.lang.Exception;
    private static java.lang.Object lambda$putVirtualMemory$1(oshi.hardware.VirtualMemory) throws java.lang.Exception;
    private static java.lang.Object lambda$putVirtualMemory$0(oshi.hardware.VirtualMemory) throws java.lang.Exception;
    private static java.lang.Object lambda$putPhysicalMemory$1(oshi.hardware.PhysicalMemory) throws java.lang.Exception;
    private static java.lang.Object lambda$putPhysicalMemory$0(oshi.hardware.PhysicalMemory) throws java.lang.Exception;
    private void lambda$putSoftware$0(oshi.software.os.OperatingSystem);
    private void lambda$putHardware$2(oshi.hardware.HardwareAbstractionLayer);
    private void lambda$putHardware$1(oshi.hardware.HardwareAbstractionLayer);
    private void lambda$putHardware$0(oshi.hardware.HardwareAbstractionLayer);
    private static java.lang.Object lambda$new$10() throws java.lang.Exception;
    private static boolean lambda$new$11(java.lang.String);
    private static java.lang.Object lambda$new$8() throws java.lang.Exception;
    private static boolean lambda$new$9(java.lang.String);
    private void lambda$new$7();
    private void lambda$new$6();
    private static java.lang.Object lambda$new$5() throws java.lang.Exception;
    private static java.lang.Object lambda$new$4() throws java.lang.Exception;
    private static java.lang.Object lambda$new$3() throws java.lang.Exception;
    private static java.lang.Object lambda$new$2() throws java.lang.Exception;
    private static java.lang.Object lambda$new$1() throws java.lang.Exception;
    private static java.lang.Object lambda$new$0() throws java.lang.Exception;
    static {};
}
```
