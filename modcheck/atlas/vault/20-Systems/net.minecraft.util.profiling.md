---
type: "system"
package: "net.minecraft.util.profiling"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.profiling

103 classes (70 top-level) across 10 packages in the processed jar; 0 changed by Loom processing; 2 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.util.profiling.Profiler|Profiler]] -- calls:7 -- by fabric-command-api-v2
- [[40-Interfaces/net.minecraft.util.profiling.ProfilerFiller|ProfilerFiller]] -- calls:7 -- by fabric-command-api-v2

## Declared inventory

### `net.minecraft.util.profiling` (15 top-level)

`ActiveProfiler`, `ContinuousProfiler`, `EmptyProfileResults`, `FilledProfileResults`, `InactiveProfiler`, `ProfileCollector`, `ProfileResults`, [[40-Interfaces/net.minecraft.util.profiling.Profiler|Profiler]], [[40-Interfaces/net.minecraft.util.profiling.ProfilerFiller|ProfilerFiller]], `ProfilerPathEntry`, `ResultField`, `SingleTickProfiler`, `TracyZoneFiller`, `Zone`, `package-info`

### `net.minecraft.util.profiling.jfr` (6 top-level)

`Environment`, `JfrProfiler`, `JvmProfiler`, `Percentiles`, `SummaryReporter`, `package-info`

### `net.minecraft.util.profiling.jfr.callback` (2 top-level)

`ProfiledDuration`, `package-info`

### `net.minecraft.util.profiling.jfr.event` (13 top-level)

`ChunkGenerationEvent`, `ChunkRegionIoEvent`, `ChunkRegionReadEvent`, `ChunkRegionWriteEvent`, `ClientFpsEvent`, `NetworkSummaryEvent`, `PacketEvent`, `PacketReceivedEvent`, `PacketSentEvent`, `ServerTickTimeEvent`, `StructureGenerationEvent`, `WorldLoadFinishedEvent`, `package-info`

### `net.minecraft.util.profiling.jfr.parse` (3 top-level)

`JfrStatsParser`, `JfrStatsResult`, `package-info`

### `net.minecraft.util.profiling.jfr.serialize` (2 top-level)

`JfrResultJsonSerializer`, `package-info`

### `net.minecraft.util.profiling.jfr.stats` (14 top-level)

`ChunkGenStat`, `ChunkIdentification`, `CpuLoadStat`, `FileIOStat`, `FpsStat`, `GcHeapStat`, `IoSummary`, `PacketIdentification`, `StructureGenStat`, `ThreadAllocationStat`, `TickTimeStat`, `TimedStat`, `TimedStatSummary`, `package-info`

### `net.minecraft.util.profiling.metrics` (6 top-level)

`MetricCategory`, `MetricSampler`, `MetricsRegistry`, `MetricsSamplerProvider`, `ProfilerMeasured`, `package-info`

### `net.minecraft.util.profiling.metrics.profiling` (6 top-level)

`ActiveMetricsRecorder`, `InactiveMetricsRecorder`, `MetricsRecorder`, `ProfilerSamplerAdapter`, `ServerMetricsSamplersProvider`, `package-info`

### `net.minecraft.util.profiling.metrics.storage` (3 top-level)

`MetricsPersister`, `RecordedDeviation`, `package-info`

