---
type: "interface"
fqcn: "net.minecraft.util.ProblemReporter$ScopedCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.ProblemReporter$ScopedCollector

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lorg/slf4j/Logger;)V` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `close()V` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.ProblemReporter$ScopedCollector extends net.minecraft.util.ProblemReporter$Collector implements java.lang.AutoCloseable {
    private final org.slf4j.Logger logger;
    public net.minecraft.util.ProblemReporter$ScopedCollector(org.slf4j.Logger);
    public net.minecraft.util.ProblemReporter$ScopedCollector(net.minecraft.util.ProblemReporter$PathElement, org.slf4j.Logger);
    public void close();
}
```
