use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;
    pub async fn on_rpc_get_friend_list_arg(
        _ctx: &mut NetworkContext<'_, RpcGetFriendListArg>,
    ) -> Result<RpcGetFriendListRet, Retcode> {
        Ok(RpcGetFriendListRet::default())
    }

    pub async fn on_rpc_get_chat_emoji_list_arg(
        _ctx: &mut NetworkContext<'_, RpcGetChatEmojiListArg>,
    ) -> Result<RpcGetChatEmojiListRet, Retcode> {
        Ok(RpcGetChatEmojiListRet::default())
    }

    pub async fn on_rpc_get_online_friends_list_arg(
        _ctx: &mut NetworkContext<'_, RpcGetOnlineFriendsListArg>,
    ) -> Result<RpcGetOnlineFriendsListRet, Retcode> {
        Ok(RpcGetOnlineFriendsListRet::default())
    }
}
