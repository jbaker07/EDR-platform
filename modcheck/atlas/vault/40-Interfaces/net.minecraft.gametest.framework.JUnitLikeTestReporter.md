---
type: "interface"
fqcn: "net.minecraft.gametest.framework.JUnitLikeTestReporter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.gametest.framework.JUnitLikeTestReporter

System: [[20-Systems/net.minecraft.gametest.framework|net.minecraft.gametest.framework]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/io/File;)V` | `` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `save(Ljava/io/File;)V` | `` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.gametest.framework.JUnitLikeTestReporter implements net.minecraft.gametest.framework.TestReporter {
    private final org.w3c.dom.Document document;
    private final org.w3c.dom.Element testSuite;
    private final com.google.common.base.Stopwatch stopwatch;
    private final java.io.File destination;
    public net.minecraft.gametest.framework.JUnitLikeTestReporter(java.io.File) throws javax.xml.parsers.ParserConfigurationException;
    private org.w3c.dom.Element createTestCase(net.minecraft.gametest.framework.GameTestInfo, java.lang.String);
    public void onTestFailed(net.minecraft.gametest.framework.GameTestInfo);
    public void onTestSuccess(net.minecraft.gametest.framework.GameTestInfo);
    public void finish();
    public void save(java.io.File) throws javax.xml.transform.TransformerException;
}
```
