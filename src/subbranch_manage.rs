use sqlx::types::BigDecimal;

use super::preludes::rocket_prelude::*;
use crate::{
    utility::GenericError,
};

pub async fn set_subbranch_asset(
    db: &mut Connection<BankManage>,
    subbranch: &str,
    new_asset: &BigDecimal,
) -> Result<(), GenericError> {
    sqlx::query("UPDATE subbranch SET subbranchAsset=? WHERE subbranchName=?")
        .bind(new_asset.to_string())
        .bind(subbranch)
        .execute(&mut **db)
        .await?;
    Ok(())
}

pub async fn query_subbranch(
    db: &mut Connection<BankManage>,
    subbranch: &str,
) -> Result<Subbranch, GenericError> {
    Ok(sqlx::query_as!(
        Subbranch,
        "SELECT * FROM subbranch WHERE subbranchName=?",
        subbranch
    )
    .fetch_one(&mut **db)
    .await?)
}
