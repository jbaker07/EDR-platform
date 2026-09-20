---
type: "interface"
fqcn: "net.minecraft.ChatFormatting"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.ChatFormatting

System: [[20-Systems/net.minecraft|net.minecraft]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `REDLnet/minecraft/ChatFormatting;` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (35, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.ChatFormatting extends java.lang.Enum<net.minecraft.ChatFormatting> {
    public static final net.minecraft.ChatFormatting BLACK;
    public static final net.minecraft.ChatFormatting DARK_BLUE;
    public static final net.minecraft.ChatFormatting DARK_GREEN;
    public static final net.minecraft.ChatFormatting DARK_AQUA;
    public static final net.minecraft.ChatFormatting DARK_RED;
    public static final net.minecraft.ChatFormatting DARK_PURPLE;
    public static final net.minecraft.ChatFormatting GOLD;
    public static final net.minecraft.ChatFormatting GRAY;
    public static final net.minecraft.ChatFormatting DARK_GRAY;
    public static final net.minecraft.ChatFormatting BLUE;
    public static final net.minecraft.ChatFormatting GREEN;
    public static final net.minecraft.ChatFormatting AQUA;
    public static final net.minecraft.ChatFormatting RED;
    public static final net.minecraft.ChatFormatting LIGHT_PURPLE;
    public static final net.minecraft.ChatFormatting YELLOW;
    public static final net.minecraft.ChatFormatting WHITE;
    public static final net.minecraft.ChatFormatting OBFUSCATED;
    public static final net.minecraft.ChatFormatting BOLD;
    public static final net.minecraft.ChatFormatting STRIKETHROUGH;
    public static final net.minecraft.ChatFormatting UNDERLINE;
    public static final net.minecraft.ChatFormatting ITALIC;
    public static final net.minecraft.ChatFormatting RESET;
    public static final char PREFIX_CODE;
    private static final java.util.regex.Pattern STRIP_FORMATTING_PATTERN;
    private final char code;
    private final java.lang.String toString;
    private static final net.minecraft.ChatFormatting[] $VALUES;
    public static net.minecraft.ChatFormatting[] values();
    public static net.minecraft.ChatFormatting valueOf(java.lang.String);
    private net.minecraft.ChatFormatting(char);
    public java.lang.String toString();
    public static java.lang.String stripFormatting(java.lang.String);
    public static net.minecraft.ChatFormatting getByCode(char);
    private static net.minecraft.ChatFormatting[] $values();
    static {};
}
```
