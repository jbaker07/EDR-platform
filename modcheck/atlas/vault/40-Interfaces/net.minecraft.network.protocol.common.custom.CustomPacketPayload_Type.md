---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@30 in `GameTestSyncPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@36 in `ClientboundAttachmentSyncPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@27 in `ClientboundRequestAcceptedAttachmentsPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@44 in `ServerboundAcceptedAttachmentsPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@23 in `Networking$OpenScreenPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@9 in `CommonRegisterPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@25 in `CommonVersionPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@7 in `RegistrationPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@20 in `RegistrationPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@11 in `FabricSplitPacketPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@26 in `ExtendedBlockParticleOptionSync$DummyPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@26 in `ClientboundCustomIngredientPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@42 in `ServerboundCustomIngredientPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@40 in `ClientboundRecipeSyncPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@38 in `ServerboundSupportedRecipeSerializersPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@21 in `SyncCompletePayload.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokespecial@13 in `RegistrySyncPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `ClientConfigurationNetworking.registerGlobalReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `ClientConfigurationNetworking.unregisterGlobalReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@10 in `ClientConfigurationNetworking.registerReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `ClientConfigurationNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `ClientPlayNetworking.registerGlobalReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@10 in `ClientPlayNetworking.registerReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `ClientPlayNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `ServerConfigurationNetworking.registerGlobalReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@5 in `ServerConfigurationNetworking.registerReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@22 in `ServerConfigurationNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `ServerPlayNetworking.registerGlobalReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@5 in `ServerPlayNetworking.registerReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@12 in `ServerPlayNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@22 in `ServerPlayNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@6 in `AbstractChanneledNetworkAddon.handle` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@7 in `CommonPacketsImpl$CommonRegisterConfigurationTask.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@7 in `CommonPacketsImpl$CommonVersionConfigurationTask.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@34 in `PayloadTypeRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@67 in `PayloadTypeRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@24 in `PayloadTypeRegistryImpl.registerLarge` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@20 in `PayloadTypeRegistryImpl.registerLarge` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@5 in `PayloadTypeRegistryImpl.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@92 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@105 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@10 in `ClientboundCustomPayloadPacketMixin.fabric_split` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@45 in `ClientboundCustomPayloadPacketMixin.fabric_split` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@85 in `IdDispatchCodecMixin.encode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@10 in `ServerboundCustomPayloadPacketMixin.fabric_split` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@45 in `ServerboundCustomPayloadPacketMixin.fabric_split` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Lnet/minecraft/resources/Identifier;
public <init>(Lnet/minecraft/resources/Identifier;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public id()Lnet/minecraft/resources/Identifier;
```
