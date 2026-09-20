---
type: "interface"
fqcn: "net.minecraft.client.KeyMapping"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.KeyMapping

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getName()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getName()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `isDown()Z` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (47, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.KeyMapping implements java.lang.Comparable<net.minecraft.client.KeyMapping> {
    private static final java.util.Map<java.lang.String, net.minecraft.client.KeyMapping> ALL;
    private static final java.util.Map<com.mojang.blaze3d.platform.InputConstants$Key, java.util.List<net.minecraft.client.KeyMapping>> MAP;
    private final java.lang.String name;
    private final com.mojang.blaze3d.platform.InputConstants$Key defaultKey;
    private final net.minecraft.client.KeyMapping$Category category;
    protected com.mojang.blaze3d.platform.InputConstants$Key key;
    private boolean isDown;
    private int clickCount;
    private final int order;
    public static void click(com.mojang.blaze3d.platform.InputConstants$Key);
    public static void set(com.mojang.blaze3d.platform.InputConstants$Key, boolean);
    private static void forAllKeyMappings(com.mojang.blaze3d.platform.InputConstants$Key, java.util.function.Consumer<net.minecraft.client.KeyMapping>);
    public static void setAll();
    public static void releaseAll();
    public static void restoreToggleStatesOnScreenClosed();
    public static void resetToggleKeys();
    public static void resetMapping();
    public net.minecraft.client.KeyMapping(java.lang.String, int, net.minecraft.client.KeyMapping$Category);
    public net.minecraft.client.KeyMapping(java.lang.String, com.mojang.blaze3d.platform.InputConstants$Type, int, net.minecraft.client.KeyMapping$Category);
    public net.minecraft.client.KeyMapping(java.lang.String, com.mojang.blaze3d.platform.InputConstants$Type, int, net.minecraft.client.KeyMapping$Category, int);
    public boolean isDown();
    public net.minecraft.client.KeyMapping$Category getCategory();
    public boolean consumeClick();
    protected void release();
    protected boolean shouldSetOnIngameFocus();
    public java.lang.String getName();
    public com.mojang.blaze3d.platform.InputConstants$Key getDefaultKey();
    public void setKey(com.mojang.blaze3d.platform.InputConstants$Key);
    public int compareTo(net.minecraft.client.KeyMapping);
    public static java.util.function.Supplier<net.minecraft.network.chat.Component> createNameSupplier(java.lang.String);
    public boolean same(net.minecraft.client.KeyMapping);
    public boolean isUnbound();
    public boolean matches(net.minecraft.client.input.KeyEvent);
    public boolean matchesMouse(net.minecraft.client.input.MouseButtonEvent);
    public boolean matches(com.mojang.blaze3d.platform.InputConstants$Key);
    public net.minecraft.network.chat.Component getTranslatedKeyMessage();
    public boolean isDefault();
    public java.lang.String saveString();
    public void setDown(boolean);
    private void registerMapping(com.mojang.blaze3d.platform.InputConstants$Key);
    public static net.minecraft.client.KeyMapping get(java.lang.String);
    public int compareTo(java.lang.Object);
    private static java.util.List lambda$registerMapping$0(com.mojang.blaze3d.platform.InputConstants$Key);
    private static net.minecraft.network.chat.Component lambda$createNameSupplier$0(java.lang.String);
    private static void lambda$set$0(boolean, net.minecraft.client.KeyMapping);
    private static void lambda$click$0(net.minecraft.client.KeyMapping);
    static {};
}
```
