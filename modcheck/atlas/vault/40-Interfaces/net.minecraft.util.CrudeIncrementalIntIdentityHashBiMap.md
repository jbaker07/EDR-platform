---
type: "interface"
fqcn: "net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `add(Ljava/lang/Object;)I` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `clear()V` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `iterator()Ljava/util/Iterator;` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `size()I` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (28, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap<K> implements net.minecraft.core.IdMap<K> {
    private static final int NOT_FOUND;
    private static final java.lang.Object EMPTY_SLOT;
    private static final float LOADFACTOR;
    private K[] keys;
    private int[] values;
    private K[] byId;
    private int nextId;
    private int size;
    private net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap(int);
    private net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap(K[], int[], K[], int, int);
    public static <A> net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap<A> create(int);
    public int getId(K);
    public K byId(int);
    private int getValue(int);
    public boolean contains(K);
    public boolean contains(int);
    public int add(K);
    private int nextId();
    private void grow(int);
    public void addMapping(K, int);
    private int hash(K);
    private int indexOf(K, int);
    private int findEmpty(int);
    public java.util.Iterator<K> iterator();
    public void clear();
    public int size();
    public net.minecraft.util.CrudeIncrementalIntIdentityHashBiMap<K> copy();
    static {};
}
```
