---
type: "interface"
fqcn: "net.minecraft.client.gui.components.ChatComponent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.ChatComponent

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addClientSystemMessage` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokevirtual@14 in `ClientSuggestionProviderMixin.sendFeedback` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (24 fields, 47 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final MAX_CHAT_HISTORY : I
private static final MESSAGE_INDENT : I
private static final BOTTOM_MARGIN : I
private static final TOOLTIP_MAX_WIDTH : I
private static final TIME_BEFORE_MESSAGE_DELETION : I
private static final DELETED_CHAT_MESSAGE : Lnet/minecraft/network/chat/Component;
public static final MESSAGE_BOTTOM_TO_MESSAGE_TOP : I
public static final QUEUE_EXPAND_ID : Lnet/minecraft/resources/Identifier;
private static final QUEUE_EXPAND_TEXT_STYLE : Lnet/minecraft/network/chat/Style;
public static final GO_TO_RESTRICTIONS_SCREEN : Lnet/minecraft/resources/Identifier;
private static final RESTRICTED_CHAT_MESSAGE : Lnet/minecraft/network/chat/Component;
private static final RESTRICTED_CHAT_MESSAGE_WITH_HOVER : Lnet/minecraft/network/chat/Component;
private final minecraft : Lnet/minecraft/client/Minecraft;
private final commandHistory : Lnet/minecraft/client/CommandHistory;
private final recentChat : Lnet/minecraft/util/ArrayListDeque;
private final allMessages : Ljava/util/List;
private final trimmedMessages : Ljava/util/List;
private chatScrollbarPos : I
private newMessageSinceScroll : Z
private latestDraft : Lnet/minecraft/client/gui/components/ChatComponent$Draft;
private preservedScreen : Lnet/minecraft/client/gui/screens/ChatScreen;
private final messageDeletionQueue : Ljava/util/List;
private visibleMessageFilter : Ljava/util/function/Predicate;
public <init>(Lnet/minecraft/client/Minecraft;)V
public tick()V
public setVisibleMessageFilter(Ljava/util/function/Predicate;)V
private forEachLine(Lnet/minecraft/client/gui/components/ChatComponent$AlphaCalculator;Lnet/minecraft/client/gui/components/ChatComponent$LineConsumer;)I
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/gui/Font;IIILnet/minecraft/client/gui/components/ChatComponent$DisplayMode;Z)V
public captureClickableText(Lnet/minecraft/client/gui/ActiveTextCollector;IILnet/minecraft/client/gui/components/ChatComponent$DisplayMode;)V
private extractRenderState(Lnet/minecraft/client/gui/components/ChatComponent$ChatGraphicsAccess;IILnet/minecraft/client/gui/components/ChatComponent$DisplayMode;)V
public clearMessages(Z)V
public addClientSystemMessage(Lnet/minecraft/network/chat/Component;)V
public addServerSystemMessage(Lnet/minecraft/network/chat/Component;)V
public addPlayerMessage(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/MessageSignature;Lnet/minecraft/client/multiplayer/chat/GuiMessageTag;)V
private addMessage(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/MessageSignature;Lnet/minecraft/client/multiplayer/chat/GuiMessageSource;Lnet/minecraft/client/multiplayer/chat/GuiMessageTag;)V
private logChatMessage(Lnet/minecraft/client/multiplayer/chat/GuiMessage;)V
private addMessageToDisplayQueue(Lnet/minecraft/client/multiplayer/chat/GuiMessage;)V
private addMessageToQueue(Lnet/minecraft/client/multiplayer/chat/GuiMessage;)V
private processMessageDeletionQueue()V
public deleteMessage(Lnet/minecraft/network/chat/MessageSignature;)V
private deleteMessageOrDelay(Lnet/minecraft/network/chat/MessageSignature;)Lnet/minecraft/client/gui/components/ChatComponent$DelayedMessageDeletion;
private static createDeletedMarker(Lnet/minecraft/client/multiplayer/chat/GuiMessage;)Lnet/minecraft/client/multiplayer/chat/GuiMessage;
public rescaleChat()V
private refreshTrimmedMessages()V
public getRecentChat()Lnet/minecraft/util/ArrayListDeque;
public addRecentChat(Ljava/lang/String;)V
public resetChatScroll()V
public scrollChat(I)V
public isChatFocused()Z
private getWidth()I
private getHeight()I
private getScale()D
public static getWidth(D)I
public static getHeight(D)I
public static defaultUnfocusedPct()D
public getLinesPerPage()I
private getLineHeight()I
public saveAsDraft(Ljava/lang/String;)V
public discardDraft()V
public createScreen(Lnet/minecraft/client/gui/components/ChatComponent$ChatMethod;Lnet/minecraft/client/gui/screens/ChatScreen$ChatConstructor;)Lnet/minecraft/client/gui/screens/ChatScreen;
public openScreen(Lnet/minecraft/client/gui/components/ChatComponent$ChatMethod;Lnet/minecraft/client/gui/screens/ChatScreen$ChatConstructor;)V
public preserveCurrentChatScreen()V
public restoreChatScreen()Lnet/minecraft/client/gui/screens/ChatScreen;
public storeState()Lnet/minecraft/client/gui/components/ChatComponent$State;
public restoreState(Lnet/minecraft/client/gui/components/ChatComponent$State;)V
private synthetic lambda$processMessageDeletionQueue$0(ILnet/minecraft/client/gui/components/ChatComponent$DelayedMessageDeletion;)Z
private static synthetic lambda$extractRenderState$1(IILnet/minecraft/client/gui/components/ChatComponent$ChatGraphicsAccess;IFLnet/minecraft/client/multiplayer/chat/GuiMessage$Line;IF)V
private static synthetic lambda$extractRenderState$0(FLorg/joml/Matrix3x2f;)V
private static synthetic lambda$new$0(Lnet/minecraft/client/multiplayer/chat/GuiMessage;)Z
static <clinit>()V
```
