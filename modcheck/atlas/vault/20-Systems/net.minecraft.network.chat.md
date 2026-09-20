---
type: "system"
package: "net.minecraft.network.chat"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat

123 classes (63 top-level) across 5 packages in the processed jar; 2 changed by Loom processing; 12 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.network.chat.ChatDecorator|ChatDecorator]] -- calls:3 -- by fabric-message-api-v1
- [[40-Interfaces/net.minecraft.network.chat.ChatType_Bound|ChatType$Bound]] -- calls:2 -- by fabric-message-api-v1
- [[40-Interfaces/net.minecraft.network.chat.CommonComponents|CommonComponents]] -- reads:26 -- by fabric-command-api-v2, fabric-data-attachment-api-v1, fabric-game-rule-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.network.chat.Component|Component]] -- calls:102 -- by fabric-client-gametest-api-v1, fabric-command-api-v2, fabric-convention-tags-v2, fabric-creative-tab-api-v1, fabric-data-attachment-api-v1, fabric-data-generation-api-v1, fabric-game-rule-api-v1, fabric-lifecycle-events-v1, fabric-registry-sync-v0, fabric-resource-loader-v0, fabric-resource-loader-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.network.chat.ComponentSerialization|ComponentSerialization]] -- reads:2 -- by fabric-menu-api-v1
- [[40-Interfaces/net.minecraft.network.chat.ComponentUtils|ComponentUtils]] -- calls:3 -- by fabric-command-api-v2, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.network.chat.FilterMask|FilterMask]] -- calls:1 -- by fabric-message-api-v1
- [[40-Interfaces/net.minecraft.network.chat.MutableComponent|MutableComponent]] -- calls:83 -- by fabric-client-gametest-api-v1, fabric-command-api-v2, fabric-data-attachment-api-v1, fabric-game-rule-api-v1, fabric-registry-sync-v0, fabric-resource-loader-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.network.chat.PlayerChatMessage|PlayerChatMessage]] -- calls:2 -- by fabric-message-api-v1
- [[40-Interfaces/net.minecraft.network.chat.Style|Style]] -- calls:2, reads:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.network.chat.TextColor|TextColor]] -- calls:3, reads:10 -- by fabric-registry-sync-v0, fabric-resource-loader-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.network.chat.contents.TranslatableContents|TranslatableContents]] -- calls:2 -- by fabric-client-gametest-api-v1, fabric-data-generation-api-v1

## Declared inventory

### `net.minecraft.network.chat` (35 top-level)

[[40-Interfaces/net.minecraft.network.chat.ChatDecorator|ChatDecorator]], `ChatType`, `ChatTypeDecoration`, `ClickEvent`, [[40-Interfaces/net.minecraft.network.chat.CommonComponents|CommonComponents]], [[40-Interfaces/net.minecraft.network.chat.Component|Component]], `ComponentContents`, [[40-Interfaces/net.minecraft.network.chat.ComponentSerialization|ComponentSerialization]], [[40-Interfaces/net.minecraft.network.chat.ComponentUtils|ComponentUtils]], [[40-Interfaces/net.minecraft.network.chat.FilterMask|FilterMask]], `FontDescription`, `FormattedText`, `HoverEvent`, `LastSeenMessages`, `LastSeenMessagesTracker`, `LastSeenMessagesValidator`, `LastSeenTrackedEntry`, `LocalChatSession`, `MessageSignature`, `MessageSignatureCache`, [[40-Interfaces/net.minecraft.network.chat.MutableComponent|MutableComponent]], `OutgoingChatMessage`, [[40-Interfaces/net.minecraft.network.chat.PlayerChatMessage|PlayerChatMessage]], `RemoteChatSession`, `ResolutionContext`, `SignableCommand`, `SignedMessageBody`, `SignedMessageChain`, `SignedMessageLink`, `SignedMessageValidator`, [[40-Interfaces/net.minecraft.network.chat.Style|Style]], `SubStringSource`, [[40-Interfaces/net.minecraft.network.chat.TextColor|TextColor]], `ThrowingComponent`, `package-info`

### `net.minecraft.network.chat.contents` (10 top-level)

`KeybindContents`, `KeybindResolver`, `NbtContents`, `ObjectContents`, `PlainTextContents`, `ScoreContents`, `SelectorContents`, [[40-Interfaces/net.minecraft.network.chat.contents.TranslatableContents|TranslatableContents]], `TranslatableFormatException`, `package-info`

### `net.minecraft.network.chat.contents.data` (6 top-level)

`BlockDataSource`, `DataSource`, `DataSources`, `EntityDataSource`, `StorageDataSource`, `package-info`

### `net.minecraft.network.chat.contents.objects` (5 top-level)

`AtlasSprite`, `ObjectInfo`, `ObjectInfos`, `PlayerSprite`, `package-info`

### `net.minecraft.network.chat.numbers` (7 top-level)

`BlankFormat`, `FixedFormat`, `NumberFormat`, `NumberFormatType`, `NumberFormatTypes`, `StyledFormat`, `package-info`

