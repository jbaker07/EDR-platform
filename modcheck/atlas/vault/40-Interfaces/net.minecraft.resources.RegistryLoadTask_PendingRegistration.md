---
type: "interface"
fqcn: "net.minecraft.resources.RegistryLoadTask$PendingRegistration"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryLoadTask$PendingRegistration

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `value()Lcom/mojang/datafixers/util/Either;` | `` | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| injects_into | `loadFromResource` | `@Inject at INVOKE Lcom/mojang/serialization/Decoder;parse(Lcom/mojang/serializat` | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.resources.RegistryLoadTask$PendingRegistration<T> extends java.lang.Record {
    private final net.minecraft.resources.ResourceKey<T> key;
    private final com.mojang.datafixers.util.Either<T, java.lang.Exception> value;
    private final net.minecraft.core.RegistrationInfo registrationInfo;
    protected net.minecraft.resources.RegistryLoadTask$PendingRegistration(net.minecraft.resources.ResourceKey<T>, com.mojang.datafixers.util.Either<T, java.lang.Exception>, net.minecraft.core.RegistrationInfo);
    public static <T> com.mojang.datafixers.util.Either<T, java.lang.Exception> loadFromResource(com.mojang.serialization.Decoder<T>, net.minecraft.resources.RegistryOps<com.google.gson.JsonElement>, net.minecraft.resources.ResourceKey<T>, net.minecraft.server.packs.resources.Resource);
    public static <T> com.mojang.datafixers.util.Either<T, java.lang.Exception> findAndLoadFromResource(com.mojang.serialization.Decoder<T>, net.minecraft.resources.RegistryOps<com.google.gson.JsonElement>, net.minecraft.resources.ResourceKey<T>, net.minecraft.resources.FileToIdConverter, net.minecraft.server.packs.resources.ResourceProvider);
    public static <T> com.mojang.datafixers.util.Either<T, java.lang.Exception> loadFromNetwork(com.mojang.serialization.Decoder<T>, net.minecraft.resources.RegistryOps<net.minecraft.nbt.Tag>, net.minecraft.resources.ResourceKey<T>, net.minecraft.nbt.Tag);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.resources.ResourceKey<T> key();
    public com.mojang.datafixers.util.Either<T, java.lang.Exception> value();
    public net.minecraft.core.RegistrationInfo registrationInfo();
    private static com.mojang.datafixers.util.Either lambda$findAndLoadFromResource$1(net.minecraft.resources.Identifier, net.minecraft.resources.ResourceKey);
    private static com.mojang.datafixers.util.Either lambda$findAndLoadFromResource$0(com.mojang.serialization.Decoder, net.minecraft.resources.RegistryOps, net.minecraft.resources.ResourceKey, net.minecraft.server.packs.resources.Resource);
}
```
