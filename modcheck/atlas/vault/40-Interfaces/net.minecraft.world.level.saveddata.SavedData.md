---
type: "interface"
fqcn: "net.minecraft.world.level.saveddata.SavedData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.saveddata.SavedData

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.level.saveddata.SavedData {
    private boolean dirty;
    public net.minecraft.world.level.saveddata.SavedData();
    public void setDirty();
    public void setDirty(boolean);
    public boolean isDirty();
}
```
