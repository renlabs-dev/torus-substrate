#[macro_export]
macro_rules! storage_value {
    ($chain:ident, $pallet:ident, $storage:ident -> $ret:ty) => {
        pub async fn $storage(&self) -> anyhow::Result<$ret> {
            let call = crate::interfaces::$chain::api::storage()
                .$pallet()
                .$storage();

            let api = match &self.block {
                Some(block_hash) => self.client.storage().at(*block_hash),
                None => self.client.storage().at_latest().await?,
            };

            Ok(api.fetch_or_default(&call).await?)
        }
    };
}

#[macro_export]
macro_rules! storage_map {
    ($chain:ident, $pallet:ident, $storage:ident -> <$key:ty, $val:ty>) => {
        paste::paste! {
            pub async fn [<$storage _get>](&self, key: &$key) -> anyhow::Result<Option<$val>> {
                let call = crate::interfaces::$chain::api::storage()
                    .$pallet()
                    .$storage(key);

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                Ok(api.fetch(&call).await?)
            }
        }

        paste::paste! {
            pub async fn [<$storage _iter>](
                &self,
            ) -> anyhow::Result<impl Stream<Item = Result<($key, $val), subxt::error::Error>>> {
                let call = crate::interfaces::$chain::api::storage()
                    .$pallet()
                    .[<$storage _iter>]();

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair =
                            <(AccountId32, $key) as Decode>::decode(&mut &res.key_bytes[..]).unwrap();
                        Ok((pair.1, res.value))
                    })
                    .into_stream();

                Ok(stream)
            }
        }
    };
}

#[macro_export]
macro_rules! storage_dmap {
    ($chain:ident, $pallet:ident, $storage:ident -> <($key1:ty, $key2:ty), $val:ty>) => {
        paste::paste! {
            pub async fn [<$storage _get>](&self, key1: &$key1, key2: &$key2) -> anyhow::Result<Option<$val>> {
                let call = crate::interfaces::$chain::api::storage()
                    .$pallet()
                    .$storage(key1, key2);

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                Ok(api.fetch(&call).await?)
            }
        }

        paste::paste! {
            pub async fn [<$storage _iter>](
                &self,
            ) -> anyhow::Result<impl Stream<Item = Result<($key1, $key2, $val), subxt::error::Error>>> {
                let call = crate::interfaces::$chain::api::storage()
                    .$pallet()
                    .[<$storage _iter>]();

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair =
                            <(AccountId32, $key1, $key2) as Decode>::decode(&mut &res.key_bytes[..]).unwrap();
                        Ok((pair.1, pair.2, res.value))
                    })
                    .into_stream();

                Ok(stream)
            }
        }

        paste::paste! {
            pub async fn [<$storage _iter_prefix>](
                &self,
                key1: &$key1
            ) -> anyhow::Result<impl Stream<Item = Result<($key1, $key2, $val), subxt::error::Error>>> {
                let call = crate::interfaces::$chain::api::storage()
                    .$pallet()
                    .[<$storage _iter1>](key1);

                let api = match &self.block {
                    Some(block_hash) => self.client.storage().at(*block_hash),
                    None => self.client.storage().at_latest().await?,
                };

                let stream = api
                    .iter(call)
                    .await?
                    .map(|res| {
                        let res = res?;
                        let pair =
                            <(AccountId32, $key1, $key2) as Decode>::decode(&mut &res.key_bytes[..]).unwrap();
                        Ok((pair.1, pair.2, res.value))
                    })
                    .into_stream();

                Ok(stream)
            }
        }
    };
}
