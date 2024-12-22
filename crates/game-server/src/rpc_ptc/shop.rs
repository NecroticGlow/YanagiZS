use super::*;

pub async fn on_rpc_get_fashion_store_data_arg(
    _: &mut NetworkContext<'_, '_, RpcGetFashionStoreDataArg>,
) -> Result<RpcGetFashionStoreDataRet, i32> {
    Ok(RpcGetFashionStoreDataRet {
        retcode: 0,
        data: FashionStoreData::default(),
    })
}

pub async fn on_rpc_get_shopping_mall_info_arg(
    _: &mut NetworkContext<'_, '_, RpcGetShoppingMallInfoArg>,
) -> Result<RpcGetShoppingMallInfoRet, i32> {
    Ok(RpcGetShoppingMallInfoRet {
        retcode: 0,
        shopping_mall_info: ShoppingMallInfo::default(),
    })
}

pub async fn on_rpc_recharge_get_item_list_arg(
    _: &mut NetworkContext<'_, '_, RpcRechargeGetItemListArg>,
) -> Result<RpcRechargeGetItemListRet, i32> {
    Ok(RpcRechargeGetItemListRet::default())
}
