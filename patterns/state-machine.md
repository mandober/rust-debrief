# State machine pattern

adapted from:   
https://hoverbear.org/2016/10/12/rust-state-machine-pattern/

Each state has discrete responsibilities and concerns. The natural way to model this in Rust is with variants of an enum. Using an enum means all the states are mutually exclusive, there can only be one state at the time. Rust's enums are "fat" - variants can hold data (a variant can be a struct), which allows us to define states and assign the data that the states will carry with them. We also need to define which transitions between states are allowed.

States are a way to reason about where a machine is in its process.
For example, a bottle filling machine: 
- it is in a WAITING state when awaiting on a new bottle.
- once it detects a bottle, it moves to the FILLING state.
- upon detecting the bottle is filled, it enters the DONE state.
- returns to the WAITING state.
- a state need not known anything about other states.

This can be modeled using an enum:

```rust
enum BottleFillerState {
  Waiting { waiting_time: std::time::Duration },
  Filling { rate: usize },
  Done,
}

struct BottleFiller {
  state: BottleFillerState,
}
```

Here, all the states are mutually exclusive, you can only be in one state at the time, but transitions are not enforced. The great thing about the state pattern is that transitions are encoded in type system - disallowed transitions cannot occur.

Requirements:
- the machine can only be in one state at the time
- each state may have associated values
- allowed transitions need to be well-defined, others forbidden
- some level of shared state is desirable
- transitioning should consume the previous state (can no longer be used)
- the type system should enforce these constraints
- no need to allocate memory for all states, at most the size of largest state
- no need for heap allocations, prefer the stack
- strive for compile-time, comprehensible error messages


## Attempt 1

```rust
enum State {
  Waiting { waiting_time: std::time::Duration },
  Filling { rate: usize },
  Done
}

struct StateMachine { state: State }

impl StateMachine {
  fn new() -> Self {
    StateMachine {
      state: State::Waiting { waiting_time: std::time::Duration::new(0, 0) }
    }
  }

  fn to_filling(&mut self) {
    self.state = match self.state {
      // Only Waiting -> Filling is valid.
      State::Waiting { .. } => State::Filling { rate: 1 },
      // The rest should fail.
      _ => panic!("Invalid state tranistion!"),
    }
  }
  // ...
}

fn main() {
  let mut state_machine = StateMachine::new();
  state_machine.to_filling();
}
```

Cons:
- Invalid transition errors happen at runtime
- only prevents invalid transitions outside of the module, since the private fields can be manipulated freely inside the module
- Every fn working with the state has to include a match statement

Pros:
- memory required to represent the state machine is eq to the size of the largest state.
- Everything happens on the stack.
- Transitioning between states has well defined semantics - works or crashes


## Attempt 2: Structures with Transitions

What if we just used a set of structs and have them all implement traits which all states should share. We could use special functions that transitioned the type into the new type.

```rust
// This is some functionality shared by all of the states
trait SharedFunctionality {
    fn get_shared_value(&self) -> usize;
}

struct Waiting {
    waiting_time: std::time::Duration,
    // Value shared by all states.
    shared_value: usize,
}

impl Waiting {
    fn new() -> Self {
        Waiting {
            waiting_time: std::time::Duration::new(0,0),
            shared_value: 0,
        }
    }
    // This fn consumes the value (Waiting instance) and transitions the type
    // from Waiting into Filling. Input type: Waiting, output type: Filling.
    // Output doesn't depend on input; input is there just to be consumed.
    fn to_filling(self) -> Filling {
        Filling {
            rate: 1,
            shared_value: 0,
        }
    }
}

impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

struct Filling {
    rate: usize,
    // Value shared by all states.
    shared_value: usize,
}

impl SharedFunctionality for Filling {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

// Since the size of the state varies, we need to wrap this in an enum
// if we want it to be usable in situations where the state machine is
// a component in a complex system.
enum State {
    Waiting(Waiting),
    Filling(Filling),
    Done(Done),
}

fn main() {
    let in_waiting_state = Waiting::new();
    let in_filling_state = in_waiting_state.to_filling();
    
    // after wrapping it in the State enum:
    let in_waiting_state = State::Waiting(Waiting::new());
    // doesn't even work since Waiting is wrapped in enum,
    // we need to match the enum to get the structs
    let in_filling_state = State::Filling(in_waiting_state.to_filling());
}
```

The idea here was that all states have some common shared values along with their own specialized values. `to_filling` function consumes a `Waiting` state and transition it into a `Filling` state.

Pros:
- Transition errors are caught at compile time.   
  For example you can't even create a Filling state accidentally without first starting with a Waiting state.
- Transition enforcement happens everywhere.   
  When a transition between states is made the old value is consumed instead of just modified.
- We don't have to use match all the time.
- Memory consumption is lean, at any given time the size is that of the state.

Cons:
- There is too much repetition.   
  You have to implement the same fns and traits for multiple structures.
- It's not always clear what values are shared between all states and which values belong to only one. Updating could be a pain due to this.
- Since the size of the state varies, we need to wrap this in an enum if we want it to be usable in situations where the state machine is a component in a complex system.

The Rust standard library defines two highly related traits: From and Into that are extremely useful and worth checking out. An important thing to note is that implementing one of these automatically implements the other. In general implementing From is preferable as it’s a bit more flexible. We can implement them very easily for our above:

```rust
impl From<Waiting> for Filling {
    fn from(val: Waiting) -> Filling {
        Filling {
            rate: 1,
            shared_value: val.shared_value,
        }
    }
}
```




## Attempt 3:


```rust
// This is our state machine.
struct BottleFillingMachine<S> {
  shared_value: usize,
  state: S
}

// The following states can be the 'S' in StateMachine<S>
struct Waiting {
  waiting_time: std::time::Duration,
}

struct Filling {
  rate: usize,
}

struct Done;

// Our Machine starts in the 'Waiting' state.
impl BottleFillingMachine<Waiting> {
  fn new(shared_value: usize) -> Self {
    BottleFillingMachine {
      shared_value: shared_value,
      state: Waiting {
        waiting_time: std::time::Duration::new(0, 0),
      }
    }
  }
}

// The following are the defined transitions between states.
impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
  fn from(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
    BottleFillingMachine {
      shared_value: val.shared_value,
      state: Filling {
        rate: 1,
      }
    }
  }
}

impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
  fn from(val: BottleFillingMachine<Filling>) -> BottleFillingMachine<Done> {
    BottleFillingMachine {
      shared_value: val.shared_value,
      state: Done,
    }
  }
}

impl From<BottleFillingMachine<Done>> for BottleFillingMachine<Waiting> {
  fn from(val: BottleFillingMachine<Done>) -> BottleFillingMachine<Waiting> {
    BottleFillingMachine {
      shared_value: val.shared_value,
      state: Waiting {
        waiting_time: std::time::Duration::new(0, 0),
      }
    }
  }
}


fn main() {
  let bottle_filler = BottleFillingMachine::new(0);

  // (Mock) Check on some shared and state-specific values
  assert_eq!(bottle_filler.state.waiting_time, std::time::Duration::new(0, 0));
  assert_eq!(bottle_filler.shared_value, 0);

  // Transition
  let bottle_filler = BottleFillingMachine::<Filling>::from(bottle_filler);

  // Can't do this anymore, it's been consumed!:
  // assert_eq!(bottle_filler.state.waiting_time, std::time::Duration::new(0, 0));

  let bottle_filler = BottleFillingMachine::<Done>::from(bottle_filler);
}
```
