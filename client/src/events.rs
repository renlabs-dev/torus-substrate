use std::marker::PhantomData;

use futures::{stream, TryStreamExt};
use subxt::{
    events::{EventDetails, StaticEvent},
    OnlineClient, PolkadotConfig,
};

use crate::client::TorusClient;

impl<C> TorusClient<C> {
    pub fn events(&self) -> Events<C> {
        Events {
            client: self.client.clone(),
            _pd: PhantomData,
        }
    }
}

pub struct Events<C> {
    client: OnlineClient<PolkadotConfig>,
    _pd: PhantomData<C>,
}

impl<C> Events<C> {
    pub async fn subscribe_unfiltered(
        &self,
    ) -> crate::Result<impl stream::Stream<Item = crate::Result<EventDetails<PolkadotConfig>>>>
    {
        let stream = self.client.blocks().subscribe_finalized().await?;

        let events_stream = stream
            .and_then(|block| async move {
                let events = block.events().await?;
                let all_events: Result<Vec<_>, _> = events.iter().collect();
                Ok(all_events?)
            })
            .map_ok(|events| stream::iter(events.into_iter().map(Ok)))
            .try_flatten();

        Ok(events_stream)
    }

    pub async fn subscribe<E: StaticEvent>(
        &self,
    ) -> crate::Result<impl stream::Stream<Item = crate::Result<E>>> {
        let stream = self.client.blocks().subscribe_finalized().await?;

        let events_stream = stream
            .and_then(|block| async move {
                let events = block.events().await?;
                let all_events: Result<Vec<_>, _> = events.find::<E>().collect();
                Ok(all_events?)
            })
            .map_ok(|events| stream::iter(events.into_iter().map(Ok)))
            .try_flatten();

        Ok(events_stream)
    }
}
