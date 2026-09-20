---
type: "interface"
fqcn: "net.minecraft.core.NonNullList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.NonNullList

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `set(ILjava/lang/Object;)Ljava/lang/Object;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set(ILjava/lang/Object;)Ljava/lang/Object;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.NonNullList<E> extends java.util.AbstractList<E> {
    private final java.util.List<E> list;
    private final E defaultValue;
    public static <E> net.minecraft.core.NonNullList<E> create();
    public static <E> net.minecraft.core.NonNullList<E> createWithCapacity(int);
    public static <E> net.minecraft.core.NonNullList<E> withSize(int, E);
    public static <E> net.minecraft.core.NonNullList<E> of(E, E...);
    protected net.minecraft.core.NonNullList(java.util.List<E>, E);
    public E get(int);
    public E set(int, E);
    public void add(int, E);
    public E remove(int);
    public int size();
    public void clear();
}
```
