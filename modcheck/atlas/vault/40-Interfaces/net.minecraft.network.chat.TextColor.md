---
type: "interface"
fqcn: "net.minecraft.network.chat.TextColor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.TextColor

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getValue()I` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (37, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.chat.TextColor {
    private static final java.lang.String CUSTOM_COLOR_PREFIX;
    public static final com.mojang.serialization.Codec<net.minecraft.network.chat.TextColor> CODEC;
    private static final java.util.Map<java.lang.String, net.minecraft.network.chat.TextColor> NAMED_COLORS;
    public static final net.minecraft.network.chat.TextColor BLACK;
    public static final net.minecraft.network.chat.TextColor DARK_BLUE;
    public static final net.minecraft.network.chat.TextColor DARK_GREEN;
    public static final net.minecraft.network.chat.TextColor DARK_AQUA;
    public static final net.minecraft.network.chat.TextColor DARK_RED;
    public static final net.minecraft.network.chat.TextColor DARK_PURPLE;
    public static final net.minecraft.network.chat.TextColor GOLD;
    public static final net.minecraft.network.chat.TextColor GRAY;
    public static final net.minecraft.network.chat.TextColor DARK_GRAY;
    public static final net.minecraft.network.chat.TextColor BLUE;
    public static final net.minecraft.network.chat.TextColor GREEN;
    public static final net.minecraft.network.chat.TextColor AQUA;
    public static final net.minecraft.network.chat.TextColor RED;
    public static final net.minecraft.network.chat.TextColor LIGHT_PURPLE;
    public static final net.minecraft.network.chat.TextColor YELLOW;
    public static final net.minecraft.network.chat.TextColor WHITE;
    private final int value;
    private final java.lang.String name;
    private net.minecraft.network.chat.TextColor(int, java.lang.String);
    private net.minecraft.network.chat.TextColor(int);
    private static net.minecraft.network.chat.TextColor named(java.lang.String, int);
    public int getValue();
    public java.lang.String serialize();
    private java.lang.String formatValue();
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
    public static net.minecraft.network.chat.TextColor fromLegacyFormat(net.minecraft.ChatFormatting);
    public static net.minecraft.network.chat.TextColor fromRgb(int);
    public static com.mojang.serialization.DataResult<net.minecraft.network.chat.TextColor> parseColor(java.lang.String);
    private static java.lang.String lambda$parseColor$2(java.lang.String);
    private static java.lang.String lambda$parseColor$1(java.lang.String);
    private static java.lang.String lambda$parseColor$0(java.lang.String);
    static {};
}
```
