use crate::types::WineColorEnum;
use bigdecimal::BigDecimal;

pub fn is_dup_wine(
    wines: &Vec<(
        i32,
        String,
        bool,
        String,
        String,
        String,
        String,
        WineColorEnum,
        BigDecimal,
        BigDecimal,
        i32,
    )>,
    wine: &(
        i32,
        String,
        bool,
        String,
        String,
        String,
        String,
        WineColorEnum,
        BigDecimal,
        BigDecimal,
        i32,
    ),
    index: usize,
) -> bool {
    for (i, wine_tmp) in wines.iter().enumerate() {
        if i != index && wine_tmp.0 == wine.0 && wine_tmp.10 >= wine.10 {
            return true;
        }
    }
    return false;
}
