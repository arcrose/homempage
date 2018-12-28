//! `Environment` is this crate's main driver.  It facilitates all of the
//! communication between a main application and its dependency applications.

use std::collections::{HashMap, VecDeque};
use std::time;

use application::{Application, Init, Update};
use ident::{Id, Pid};
use message::Message;

const ASYNC_WAIT_TIME_NANOSECONDS: u32 = 10_000_000;

/// Manages the state of an application and facilitates message passing between
/// it and dependent applications.
/// 
/// `Environment` also automatically polls applications for completed
/// asynchronous work by sending them an `async-check` message that contains no
/// data.  An application's `update` method can respond to these messages
/// with `Update::NotReady` to indicate that their asynchronous task is not yet
/// complete.
///
/// `Environment` also responds to messages with the identifier
/// `"request-dependency"` and associated data of type `RequestDependency` with
/// a `"dependency-lookup"` with associated data of type `DependencyLookup`.
pub struct Environment<App> {
    application: App,
    application_id_to_pid: HashMap<Id, Pid>,
    other_applications: HashMap<Pid, Box<Application>>,
    mailbox: VecDeque<Message>,
    async_tracker: HashMap<Pid, AsyncPollState>,
}

/// A builder to configure new `Environment`s.
pub struct EnvironmentBuilder<App> {
    application: App,
    dependencies: Vec<(Id, Box<Application>)>,
}

/// Represents each of the states that an application can be in with regards
/// to completing an asynchronous task, from the perspective of an
/// `Environment`.
///
/// An application that is `NotPinged` will be scheduled to have an
/// `async-check` sent to it as soon as possible and move into `WasPinged`.
///
/// An application that has indicated it is `Update::NotReady` in response to
/// an `async-check` will be moved into a the `Waiting` state.  While in the
/// `Waiting` state for some period of time or in the `WasPinged` state,
/// the `Environment` will not continue to send `async-check` to an application.
#[derive(Debug)]
enum AsyncPollState {
    NotPinged,
    WasPinged,
    Waiting {
        last_ping: time::Instant,
        wait: time::Duration,
    }
}

/// The data type for a `request-dependency` message sent to an `Environment`.
///
/// `request-dependency` can be sent to an `Environment` to get the `Pid` of a
/// dependency application registered with a gven identifier as a `String`.
#[derive(Serialize, Deserialize)]
pub struct RequestDependency {
    pub id: String,
}

/// The data type for a `dependency-lookup` message sent in response to a
/// `request-dependency` message to an `Environment`.
///
/// It contains the identifier of the dependency queried and, if such a 
/// dependency exists, the `Pid` that messages can be sent to in order to
/// communicate with it.
#[derive(Serialize, Deserialize)]
pub struct DependencyLookup {
    pub id: String,
    pub pid: Option<Pid>,
}


impl<App: Application> EnvironmentBuilder<App> {
    /// Construct a new `EnvironmentBuilder` to configure an `Environment` that
    /// will run `app` as the main application.
    pub fn new(app: App) -> Self {
        EnvironmentBuilder {
            application: app,
            dependencies: vec![],
        }
    }

    /// Register a new dependency application with a given identifier.
    pub fn dep<I: Into<Id>>(mut self, identifier: I, app: Box<Application>) -> Self {
        self.dependencies.push( (identifier.into(), app) );
        self
    }

    /// Finalize the construction of the `Environment`.
    pub fn build(self) -> Environment<App> {
        Environment::new(self.application, self.dependencies)
    }
}


impl<App: Application> Environment<App> {
    /// Constructs a new `Environment` with a main application and collection of
    /// registered dependencies.
    fn new(app: App, deps: Vec<(Id, Box<Application>)>) -> Self {
        let map: HashMap<Id, Pid> = deps
            .iter()
            .enumerate()
            .map(|(index, &(ref id, _))| (id.clone(), Pid(3 + index as u64)))
            .collect();

        let deps: HashMap<Pid, Box<Application>> = deps
            .into_iter()
            .enumerate()
            .map(|(index, (_, app))| (Pid( 3 + index as u64 ), app))
            .collect();
        
        let mut trackers: HashMap<Pid, AsyncPollState> = HashMap::new();

        for (&pid, _) in deps.iter() {
            trackers.insert(pid, AsyncPollState::NotPinged);
        }

        trackers.insert(Pid::app(), AsyncPollState::NotPinged);

        Environment {
            application: app,
            application_id_to_pid: map,
            other_applications: deps,
            mailbox: VecDeque::new(),
            async_tracker: trackers,
        }
    }

    /// Execute the `Environment`'s main application until it terminates by
    /// returning an error.  Returns the final state of the main application
    /// along with the error.
    ///
    /// This function will block the current thread until completion.
    pub fn start(mut self) -> (App, Box<::std::error::Error>) {
        if let Init::Messages(mut init_msgs) = self.init() {
            for msg in init_msgs.iter_mut() {
                msg.sender = Pid::env();

                if msg.recipient == Pid::me() {
                    msg.recipient = Pid::env();
                }
            }

            self.mailbox.extend(init_msgs);
        }

        if let Init::Messages(mut init_msgs) = self.application.init() {
            for msg in init_msgs.iter_mut() {
                msg.sender = Pid::app();

                if msg.recipient == Pid::me() {
                    msg.recipient = Pid::app();
                }
            }

            self.mailbox.extend(init_msgs);
        }

        for (&id, dep) in self.other_applications.iter_mut() {
            if let Init::Messages(mut init_msgs) = dep.init() {
                for msg in init_msgs.iter_mut() {
                    msg.sender = id;

                    if msg.recipient == Pid::me() {
                        msg.recipient = id;
                    }
                }

                self.mailbox.extend(init_msgs);
            }
        }

        'readmsg: loop {
            {
                let msgs = self.check_for_completed_async_work();
                self.mailbox.extend(msgs);
            }

            if let Some(inbound_msg) = self.mailbox.pop_front() {
                let sender = inbound_msg.sender;
                let recipient = inbound_msg.recipient;
                let msg_is_ping = inbound_msg.id.as_str() == "async-check";

                let update = self.other_applications.get_mut(&recipient)
                    .map(|dep| dep.update(inbound_msg.clone()))
                    .unwrap_or_else(||
                        if recipient == Pid::app() {
                            self.application.update(inbound_msg)
                        } else if recipient == Pid::env() {
                            self.update(inbound_msg)
                        } else {
                            Update::NoMessages
                        });

                match update {
                    Update::Messages(mut msgs) => {
                        if msg_is_ping {
                            self.set_app_async_state(&recipient, AsyncPollState::NotPinged);
                        }

                        for msg in msgs.iter_mut() {
                            msg.sender = recipient;

                            if msg.recipient == Pid::me() {
                                msg.recipient = sender;
                            }
                        }
                        
                        self.mailbox.extend(msgs);
                    },

                    Update::NotReady => {
                        self.set_app_async_state(&recipient, AsyncPollState::Waiting {
                            last_ping: time::Instant::now(),
                            wait: time::Duration::new(0, ASYNC_WAIT_TIME_NANOSECONDS),
                        });
                    },

                    Update::Error(err) => {
                        println!("ERROR: {}", err);
                        return (self.application, err);
                    },

                    _ => {
                        if msg_is_ping {
                            self.set_app_async_state(&recipient, AsyncPollState::NotPinged);
                        }
                    },
                }
            } else {
                // Didnt' read a message.
                ::std::thread::sleep(time::Duration::new(0, ASYNC_WAIT_TIME_NANOSECONDS));
            }
        }
    }

    fn set_app_async_state(&mut self, process: &Pid, state: AsyncPollState) {
        if let Some(async_status) = self.async_tracker.get_mut(process) {
            *async_status = state;
        }
    }

    /// Inspects the async. state of each application to determine which should
    /// have an `async-check` sent to them and moves each async. state forward
    /// where appropriate.
    fn check_for_completed_async_work(&mut self) -> Vec<Message> {
        let mut messages = Vec::new();
    
        for (&pid, async_status) in self.async_tracker.iter_mut() {
            let (next_state, should_msg) = match *async_status {
                AsyncPollState::NotPinged =>
                    (AsyncPollState::WasPinged, true),

                AsyncPollState::WasPinged =>
                    (AsyncPollState::WasPinged, false),

                AsyncPollState::Waiting{ last_ping, wait } => {
                    let since_ping = last_ping.elapsed();

                    let state = match since_ping > wait {
                        true  => AsyncPollState::NotPinged,
                        false => AsyncPollState::Waiting{ last_ping, wait },
                    };

                    (state, false)
                },
            };

            if should_msg {
                messages.push(Message::empty(pid, "async-check"));
            }

            *async_status = next_state;
        }

        messages
    }
}


impl<App: Application> Application for Environment<App> {
    fn init(&mut self) -> Init {
        Init::Messages(vec![
            Message::new(Pid::app(), "dependencies", &self.application_id_to_pid),
        ])
    }

    fn update(&mut self, msg: Message) -> Update {
        let mut messages = Vec::new();

        if msg.id.as_str()  == "request-dependency" {
            let lookup_result = msg.get_data::<RequestDependency>()
                .ok()
                .map(|request| {
                    let id = Id::new(request.id);

                    self.application_id_to_pid
                        .get(&id)
                        .map(Clone::clone)
                        .map(|pid| (id.clone(), Some(pid)))
                        .unwrap_or((id.clone(), None))
                });

            match lookup_result {
                Some((id, Some(pid))) =>
                    messages.push(Message::new(msg.sender, "dependency-lookup", DependencyLookup {
                        id: id.0,
                        pid: Some(pid),
                    })),

                Some((id, None)) =>
                    messages.push(Message::new(msg.sender, "dependency-lookup", DependencyLookup {
                        id: id.0,
                        pid: None,
                    })),

                None =>
                    (),
            };
        }

        Update::Messages(messages)
    }
}
