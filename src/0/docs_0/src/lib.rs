//! 本クレートについて
//! 
//! 　このクレートは`cargo doc`を使ってみただけである。
//! 
//! * https://doc.rust-jp.rs/book/second-edition/ch14-02-publishing-to-crates-io.html

/// 1を加算した値を返す。
///
/// # 例
///
/// ```
/// assert_eq!(add_one(0), 1);
/// ```
pub fn add_one(x: i32) { x + 1 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_one() {
        assert_eq!(add_one(0), 1);
    }
}
