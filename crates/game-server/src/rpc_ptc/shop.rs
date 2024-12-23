use evelyn_codegen::handlers;

#[handlers]
mod handlers {
    use crate::rpc_ptc::*;
    pub async fn on_rpc_get_fashion_store_data_arg(
        _ctx: &mut NetworkContext<'_, RpcGetFashionStoreDataArg>,
    ) -> Result<RpcGetFashionStoreDataRet, Retcode> {
        Ok(RpcGetFashionStoreDataRet {
            retcode: Retcode::Succ,
            data: FashionStoreData::default(),
        })
    }

    pub async fn on_rpc_get_shopping_mall_info_arg(
        _ctx: &mut NetworkContext<'_, RpcGetShoppingMallInfoArg>,
    ) -> Result<RpcGetShoppingMallInfoRet, Retcode> {
        Ok(RpcGetShoppingMallInfoRet {
            retcode: Retcode::Succ,
            shopping_mall_info: ShoppingMallInfo::default(),
        })
    }

    pub async fn on_rpc_recharge_get_item_list_arg(
        _ctx: &mut NetworkContext<'_, RpcRechargeGetItemListArg>,
    ) -> Result<RpcRechargeGetItemListRet, Retcode> {
        Ok(RpcRechargeGetItemListRet::default())
    }
}
