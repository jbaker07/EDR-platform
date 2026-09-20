---
type: "interface"
fqcn: "net.minecraft.world.level.gamerules.GameRuleMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gamerules.GameRuleMap

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `TYPELnet/minecraft/world/level/saveddata/SavedDataType;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.gamerules.GameRuleMap extends net.minecraft.world.level.saveddata.SavedData {
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.gamerules.GameRuleMap> CODEC;
    public static final net.minecraft.world.level.saveddata.SavedDataType<net.minecraft.world.level.gamerules.GameRuleMap> TYPE;
    private final it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.world.level.gamerules.GameRule<?>, java.lang.Object> map;
    private net.minecraft.world.level.gamerules.GameRuleMap(it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.world.level.gamerules.GameRule<?>, java.lang.Object>);
    private static net.minecraft.world.level.gamerules.GameRuleMap ofTrusted(java.util.Map<net.minecraft.world.level.gamerules.GameRule<?>, java.lang.Object>);
    public static net.minecraft.world.level.gamerules.GameRuleMap of();
    public static net.minecraft.world.level.gamerules.GameRuleMap of(java.util.stream.Stream<net.minecraft.world.level.gamerules.GameRule<?>>);
    public static net.minecraft.world.level.gamerules.GameRuleMap copyOf(net.minecraft.world.level.gamerules.GameRuleMap);
    public boolean has(net.minecraft.world.level.gamerules.GameRule<?>);
    public <T> T get(net.minecraft.world.level.gamerules.GameRule<T>);
    public <T> void set(net.minecraft.world.level.gamerules.GameRule<T>, T);
    public <T> void reset(net.minecraft.world.level.gamerules.GameRule<T>);
    public <T> T remove(net.minecraft.world.level.gamerules.GameRule<T>);
    public java.util.Set<net.minecraft.world.level.gamerules.GameRule<?>> keySet();
    public int size();
    public java.lang.String toString();
    public net.minecraft.world.level.gamerules.GameRuleMap withOther(net.minecraft.world.level.gamerules.GameRuleMap);
    public void setFromIf(net.minecraft.world.level.gamerules.GameRuleMap, java.util.function.Predicate<net.minecraft.world.level.gamerules.GameRule<?>>);
    private static <T> void setGameRule(net.minecraft.world.level.gamerules.GameRuleMap, net.minecraft.world.level.gamerules.GameRule<T>, net.minecraft.world.level.gamerules.GameRuleMap);
    private it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.world.level.gamerules.GameRule<?>, java.lang.Object> map();
    public boolean equals(java.lang.Object);
    public int hashCode();
    private static boolean lambda$withOther$0(net.minecraft.world.level.gamerules.GameRule);
    private static void lambda$of$0(it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap, net.minecraft.world.level.gamerules.GameRule);
    static {};
}
```
