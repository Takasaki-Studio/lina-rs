use sqlx::any::AnyArguments;

pub struct QueryData<'a> {
    pub sql: String,
    pub args: AnyArguments<'a>,
}

impl QueryData<'_> {}

#[macro_export]
macro_rules! values {
    ($( $key:expr => $val:expr ),* $(,)?) => {{
        use sqlx::any::AnyArguments;
        use lina_rs::sqlx::QueryData;

        let mut keys = Vec::new();
        let mut args = AnyArguments::default();
        let mut placeholders = Vec::new();

        $(
            keys.push($key);
            args.add($val).unwrap();
            placeholders.push("?");
        )*

        QueryData {
            sql: format!(
                "({}) VALUES ({})",
                keys.join(", "),
                placeholders.join(", ")
            ),
            args,
        }
    }};
}
