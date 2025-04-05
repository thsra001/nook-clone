
// thanks to https://github.com/rlidwka/bevy_mod_physx/blob/3baa1ea0701aaea8faedf3281e488b6f369cfe7b/src/events.rs#L3
use bevy::prelude::*;
use std::sync::mpsc::{Receiver,Sender};
use std::sync::Mutex;

#[derive(Resource, Deref, DerefMut)]
struct ChannelReceiver<T>(Mutex<Receiver<T>>);
#[derive(Resource)]
pub struct ChannelSender<T>(pub Sender<T>);
pub trait AppExtensions {
    // Allows you to create bevy events using mpsc Sender
    fn add_event_channel<T: Event>(&mut self, receiver: Receiver<T>) -> &mut Self;
}

impl AppExtensions for App {
    fn add_event_channel<T: Event>(&mut self, receiver: Receiver<T>) -> &mut Self {
        assert!(
            !self.world().contains_resource::<ChannelReceiver<T>>(),
            "this event channel is already initialized",
        );

        self.add_event::<T>();
        self.add_systems(
            First,
            channel_to_event::<T>
                //.after(Events::<T>::)
                
        );
        self.insert_resource(ChannelReceiver(Mutex::new(receiver)));
        self
    }
}

fn channel_to_event<T: 'static + Send + Sync + Event>(
    receiver: Res<ChannelReceiver<T>>,
    mut writer: EventWriter<T>,
) {
    // this should be the only system working with the receiver,
    // thus we always expect to get this lock
    let events = receiver.lock().expect("unable to acquire mutex lock");

    writer.send_batch(events.try_iter());
}