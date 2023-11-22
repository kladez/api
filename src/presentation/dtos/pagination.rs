use std::fmt::{
    Display,
    Formatter,
};

use serde::{
    de::{
        DeserializeOwned,
        MapAccess,
        Visitor,
    },
    Deserialize,
    Deserializer,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Order {
    Asc,
    Desc,
}

impl Display for Order {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(match self {
            Order::Asc => "ASC",
            Order::Desc => "DESC",
        })
    }
}

#[derive(Debug)]
pub struct Pagination<T>
where
    T: Default + Display + DeserializeOwned,
{
    pub page: i64,
    pub size: i64,
    pub order_by: T,
    pub order: Order,
}

const DEFAULT_PAGE: i64 = 1;
const DEFAULT_SIZE: i64 = 10;
const DEFAULT_ORDER: Order = Order::Asc;

impl<'de, T> Deserialize<'de> for Pagination<T>
where
    T: Default + Display + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PaginationVisitor<T> {
            _marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for PaginationVisitor<T>
        where
            T: Default + Display + DeserializeOwned,
        {
            type Value = Pagination<T>;

            fn expecting(
                &self,
                formatter: &mut Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("struct Pagination")
            }

            fn visit_map<A>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut page = None;
                let mut size = None;
                let mut order_by = None;
                let mut order = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "page" => page = map.next_value()?,
                        "size" => size = map.next_value()?,
                        "order_by" => order_by = map.next_value()?,
                        "order" => order = map.next_value()?,
                        _ => return Err(serde::de::Error::unknown_field(key, &["page", "size", "order_by", "order"])),
                    }
                }

                let page = page.unwrap_or(DEFAULT_PAGE);
                let size = size.unwrap_or(DEFAULT_SIZE);
                let order_by = order_by.unwrap_or_else(T::default);
                let order = order.unwrap_or(DEFAULT_ORDER);

                Ok(Pagination {
                    page,
                    size,
                    order_by,
                    order,
                })
            }
        }

        deserializer.deserialize_struct(
            "Pagination",
            &["page", "size", "order_by", "order"],
            PaginationVisitor {
                _marker: std::marker::PhantomData,
            },
        )
    }
}
