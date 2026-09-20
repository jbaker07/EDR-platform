---
type: "interface"
fqcn: "net.minecraft.client.Options$3"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.Options$3

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `process(Ljava/lang/String;Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang/Object;` | `@ModifyArg at INVOKE Ljava/util/function/Function;apply(Ljava/lang/Object;)Ljava` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
class net.minecraft.client.Options$3 implements net.minecraft.client.Options$FieldAccess {
    final java.io.PrintWriter val$writer;
    net.minecraft.client.Options$3();
    public void writePrefix(java.lang.String);
    public <T> void process(java.lang.String, net.minecraft.client.OptionInstance<T>);
    public int process(java.lang.String, int);
    public boolean process(java.lang.String, boolean);
    public java.lang.String process(java.lang.String, java.lang.String);
    public float process(java.lang.String, float);
    public <T> T process(java.lang.String, T, java.util.function.Function<java.lang.String, T>, java.util.function.Function<T, java.lang.String>);
    private void lambda$process$1(java.lang.String, java.io.PrintWriter, com.google.gson.JsonElement);
    private static void lambda$process$0(net.minecraft.client.OptionInstance, com.mojang.serialization.DataResult$Error);
}
```
