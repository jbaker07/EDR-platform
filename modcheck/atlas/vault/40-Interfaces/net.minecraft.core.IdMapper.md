---
type: "interface"
fqcn: "net.minecraft.core.IdMapper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.IdMapper

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `addMapping(Ljava/lang/Object;I)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.IdMapper<T> implements net.minecraft.core.IdMap<T> {
    private int nextId;
    private final it.unimi.dsi.fastutil.objects.Reference2IntMap<T> tToId;
    private final java.util.List<T> idToT;
    public net.minecraft.core.IdMapper();
    public net.minecraft.core.IdMapper(int);
    public void addMapping(T, int);
    public void add(T);
    public int getId(T);
    public final T byId(int);
    public java.util.Iterator<T> iterator();
    public boolean contains(int);
    public int size();
}
```
