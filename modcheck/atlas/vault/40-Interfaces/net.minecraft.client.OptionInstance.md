---
type: "interface"
fqcn: "net.minecraft.client.OptionInstance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.OptionInstance

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `get()Ljava/lang/Object;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `set(Ljava/lang/Object;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (36, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.OptionInstance<T> {
    private static final org.slf4j.Logger LOGGER;
    public static final net.minecraft.client.OptionInstance$Enum<java.lang.Boolean> BOOLEAN_VALUES;
    public static final net.minecraft.client.OptionInstance$CaptionBasedToString<java.lang.Boolean> BOOLEAN_TO_STRING;
    public static final net.minecraft.client.OptionInstance$ValueUpdateListener<java.lang.Object> NO_ACTION;
    private final net.minecraft.client.OptionInstance$TooltipSupplier<T> tooltip;
    private final java.util.function.Function<T, net.minecraft.network.chat.Component> toString;
    private final net.minecraft.client.OptionInstance$ValueSet<T> values;
    private final com.mojang.serialization.Codec<T> codec;
    private final T initialValue;
    private final net.minecraft.client.OptionInstance$ValueUpdateListener<? super T> onValueUpdate;
    private final net.minecraft.network.chat.Component caption;
    private T value;
    public static net.minecraft.client.OptionInstance<java.lang.Boolean> createBoolean(java.lang.String, boolean, net.minecraft.client.OptionInstance$ValueUpdateListener<? super java.lang.Boolean>);
    public static net.minecraft.client.OptionInstance<java.lang.Boolean> createBoolean(java.lang.String, boolean);
    public static net.minecraft.client.OptionInstance<java.lang.Boolean> createBoolean(java.lang.String, net.minecraft.client.OptionInstance$TooltipSupplier<java.lang.Boolean>, boolean);
    public static net.minecraft.client.OptionInstance<java.lang.Boolean> createBoolean(java.lang.String, net.minecraft.client.OptionInstance$TooltipSupplier<java.lang.Boolean>, boolean, net.minecraft.client.OptionInstance$ValueUpdateListener<? super java.lang.Boolean>);
    public static net.minecraft.client.OptionInstance<java.lang.Boolean> createBoolean(java.lang.String, net.minecraft.client.OptionInstance$TooltipSupplier<java.lang.Boolean>, net.minecraft.client.OptionInstance$CaptionBasedToString<java.lang.Boolean>, boolean, net.minecraft.client.OptionInstance$ValueUpdateListener<? super java.lang.Boolean>);
    public net.minecraft.client.OptionInstance(java.lang.String, net.minecraft.client.OptionInstance$TooltipSupplier<T>, net.minecraft.client.OptionInstance$CaptionBasedToString<T>, net.minecraft.client.OptionInstance$ValueSet<T>, T, net.minecraft.client.OptionInstance$ValueUpdateListener<? super T>);
    public net.minecraft.client.OptionInstance(java.lang.String, net.minecraft.client.OptionInstance$TooltipSupplier<T>, net.minecraft.client.OptionInstance$CaptionBasedToString<T>, net.minecraft.client.OptionInstance$ValueSet<T>, com.mojang.serialization.Codec<T>, T, net.minecraft.client.OptionInstance$ValueUpdateListener<? super T>);
    public static <T> net.minecraft.client.OptionInstance$TooltipSupplier<T> noTooltip();
    public static <T> net.minecraft.client.OptionInstance$TooltipSupplier<T> cachedConstantTooltip(net.minecraft.network.chat.Component);
    public net.minecraft.client.gui.components.AbstractWidget createButton(net.minecraft.client.Options);
    public net.minecraft.client.gui.components.AbstractWidget createButton(net.minecraft.client.Options, int, int, int);
    public net.minecraft.client.gui.components.AbstractWidget createButton(net.minecraft.client.Options, int, int, int, net.minecraft.client.OptionInstance$ValueUpdateListener<? super T>);
    public T get();
    public com.mojang.serialization.Codec<T> codec();
    public java.lang.String toString();
    public void set(T);
    public net.minecraft.client.OptionInstance$ValueSet<T> values();
    private java.lang.Object lambda$set$0(java.lang.Object);
    private static net.minecraft.client.gui.components.Tooltip lambda$cachedConstantTooltip$0(net.minecraft.network.chat.Component, java.lang.Object);
    private static net.minecraft.client.gui.components.Tooltip lambda$noTooltip$0(java.lang.Object);
    private net.minecraft.network.chat.Component lambda$new$0(net.minecraft.client.OptionInstance$CaptionBasedToString, java.lang.Object);
    private static void lambda$static$1(java.lang.Object);
    private static net.minecraft.network.chat.Component lambda$static$0(net.minecraft.network.chat.Component, java.lang.Boolean);
    static {};
}
```
