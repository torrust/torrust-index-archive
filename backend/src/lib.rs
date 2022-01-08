pub mod handlers;
pub mod models;
pub mod utils;
pub mod database;
pub mod config;
pub mod errors;
pub mod common;
pub mod auth;
pub mod tracker;
pub mod mailer;

trait AsCSV {
    fn as_csv<T>(&self) -> Result<Option<Vec<T>>, ()>
        where
            T: std::str::FromStr;
}

impl<S> AsCSV for Option<S>
    where
        S: AsRef<str>,
{
    fn as_csv<T>(&self) -> Result<Option<Vec<T>>, ()>
        where
            T: std::str::FromStr,
    {
        match self {
            Some(ref s) if !s.as_ref().trim().is_empty() => {
                let mut acc = vec![];
                for s in s.as_ref().split(',') {
                    let item = s.trim().parse::<T>().map_err(|_| ())?;
                    acc.push(item)
                }
                if acc.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(acc))
                }
            }
            _ => Ok(None),
        }
    }
}
