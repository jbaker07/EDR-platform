---
type: "interface"
fqcn: "net.minecraft.core.Direction$Axis"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Direction$Axis

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `isVertical()Z` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `ordinal()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (28, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.Direction$Axis extends java.lang.Enum<net.minecraft.core.Direction$Axis> implements java.util.function.Predicate<net.minecraft.core.Direction>, net.minecraft.util.StringRepresentable {
    public static final net.minecraft.core.Direction$Axis X;
    public static final net.minecraft.core.Direction$Axis Y;
    public static final net.minecraft.core.Direction$Axis Z;
    public static final net.minecraft.core.Direction$Axis[] VALUES;
    public static final net.minecraft.util.StringRepresentable$EnumCodec<net.minecraft.core.Direction$Axis> CODEC;
    private final java.lang.String name;
    private static final net.minecraft.core.Direction$Axis[] $VALUES;
    public static net.minecraft.core.Direction$Axis[] values();
    public static net.minecraft.core.Direction$Axis valueOf(java.lang.String);
    private net.minecraft.core.Direction$Axis(java.lang.String);
    public static net.minecraft.core.Direction$Axis byName(java.lang.String);
    public java.lang.String getName();
    public boolean isVertical();
    public boolean isHorizontal();
    public net.minecraft.core.Direction getPositive();
    public net.minecraft.core.Direction getNegative();
    public net.minecraft.core.Direction[] getDirections();
    public java.lang.String toString();
    public static net.minecraft.core.Direction$Axis getRandom(net.minecraft.util.RandomSource);
    public boolean test(net.minecraft.core.Direction);
    public net.minecraft.core.Direction$Plane getPlane();
    public java.lang.String getSerializedName();
    public int choose(int, int, int);
    public double choose(double, double, double);
    public boolean choose(boolean, boolean, boolean);
    public boolean test(java.lang.Object);
    private static net.minecraft.core.Direction$Axis[] $values();
    static {};
}
```
