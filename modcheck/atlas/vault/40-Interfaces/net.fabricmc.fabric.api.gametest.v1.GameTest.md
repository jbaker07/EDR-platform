---
type: "interface"
fqcn: "net.fabricmc.fabric.api.gametest.v1.GameTest"
module: "fabric-gametest-api-v1"
sha256: "1bd8282a95da3822a15d7468d3542f493b9186d214ef10f938b0006319409647"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.gametest.v1.GameTest

Module: [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] -- kind: interface

```java
public abstract java.lang.String environment()
public abstract java.lang.String dimension()
public abstract java.lang.String structure()
public abstract int maxTicks()
public abstract int setupTicks()
public abstract boolean required()
public abstract net.minecraft.world.level.block.Rotation rotation()
public abstract boolean manualOnly()
public abstract int maxAttempts()
public abstract int requiredSuccesses()
public abstract boolean skyAccess()
public abstract int padding()
```
