# Object-Oriented Design Pattern

The state pattern is an object-oriented design pattern. In this pattern
- a value has internal state represented by a set of state objects
- value's behavior changes based on the internal state
- state objects share functionality
- state objects are responsible:
  - for their own behavior
  - for governing when they should change into another state.
- the value that holds state objects knows nothing about 
  - the different behavior of each state or 
  - when to transition between states.

When the program's requirements change, we can add more states, and we never need to update the code of the value holding the state or the code that uses the value; only the rules for a particular state object need to be updated.


## States
This workshop is about modeling a workflow of a blogging platform that has the following functionality:
0. new Post is instantiated as an empty post
  - the only accepted action: `new()`
  - `Post --> (new) --> DRAFT (entry state)`
1. Post in the _draft_ state accepts:
  - adding of text to its content: `set()` action
  - requesting a review: `request_review()` action [^1]
2. the Post transitions into the _pending_ state.
  - `Post in DRAFT --> (request review) --> Post PENDING`
- Once the post is approved, it gets published
  - Post in pending state accepts `approve` action
  `Post PENDING --> (approved) --> Post PUBLISHED`
- only published posts return ("get") content
- other actions should have no effect

[^1]: The way things are set now, the post can request review without first accepting some text for its content. To enforce addition of text before requesting a review, we need another state.


```
entry   => new        => EMPTY
EMPTY   => add_text   => DRAFT
DRAFT   => req_review => PENDING
PENDING => approve    => PUBLISHED

entry   req.review         approved
  DRAFT ---------> PENDING -------> PUBLISHED
```


## Constraints
- only the `Post` type is public,
- `Post` holds a value, one of the 3 state objects that
   repr the post's states: DRAFT, PENDING, PUBLISHED
- transition between states is managed internally in `Post`
  - transitions are triggered by actions on the `Post`
  - transitions cannot be triggered directly,
    only by specific actions on the particular state
    e.g. post can't go from draft to published, etc.


## Components
1. public `Post` struct:
  - field `state: Option<Box<State>>`for state objects
  - field `content: String` for text content
  - `impl Post`:
    - `new` ass.fn
    - `get` method
    - `set` method
    - public `request_review` method
    - public `approve` method

2. trait `State` for shared behavior
  - `request_review` method
  - `approve` method

3. `Draft` struct, state object
  - `impl State for Draft`
    - `request_review` method: _allowed_
    - `approve` method: _disallowed_

4. `Pending` struct, state object
  - `impl State for Pending`
    - `request_review` method: _disallowed_
    - `approve` method: _allowed_

5. `Published` struct, state object
  - `impl State for Published`
    - `request_review` method: _disallowed_
    - `approve` method: _disallowed_

Note: this design is obviously not ideal as we have a trait that all structs implement and therefore must provide all the traits methods, regardless if they need them or not.


## Implementation: step 1
The `State` trait defines the behavior shared by different post states, and the `Draft`, `Pending`, and `Published` states will all implement the `State` trait.

```rust
trait State {}

pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

struct Draft {}

impl State for Draft {}

fn main() {
    let mut post = Post::new();

    post.set("poison in the human machine");
    // get should still return no content
    assert_eq!("", post.get());

    post.request_review();
    // get should still return no content
    assert_eq!("", post.get());

    post.approve();
    // when approved, the post should get published.
    // get should now return the contents (for printing)
    assert_eq!("poison in the human machine", post.get());
}
```

When we create a new `Post`, we set its `state` field to a `Some` value that holds a `Box`. This `Box` points to a new instance of the `Draft` struct. This ensures whenever we create a new instance of `Post`, it’ll start out as a draft. Because the state field of `Post` is private, there’s no way to create a `Post` in any other state.


## Setting Post Content
In the `Post::new` function, we set the content field to a new, empty String. We want to be able to call `set` and pass it a `&str` that's added to the `content` of the `Post`. We implement this as a method rather than exposing the content field as public. We can impl a method later that will control how the content field’s data is read.

This behavior doesn’t depend on the state the post is in, so it’s not a part of the state pattern. The `set` method doesn’t interact with the `state` field at all, but it is part of the behavior we want to support.

```rust
impl Post {
    pub fn set(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

## Draft
We want `get` method to always return an empty &str, since the post is in the Draft state until approved. We're going to change this later once we implement the ability to change a post’s state.

```rust
impl Post {
    pub fn get(&mut self) -> &str {
      // return an empty &str for now
      ""
    }
}
```

## Changing States
Next, we need to add functionality to `request_review`, which should change the post's state from `Draft` to `Pending`. 

We want to give `Post` a public method named `request_review` that takes a `&mut self`. Then we’re going to call an internal `request_review` method on the current state of `Post`, and it will consume the current state, returning a new one.

```rust
trait State {
    fn request_review(self: Box<Self>) -> Box<State>;
}
```

We first add the internal `request_review` method to the `State` trait; all types that implement the trait will now need to implement this method.

Rather than taking an instance (an instance of the type that impl this trait a.k.a self) as `self` / `&self` / `&mut self` (which are shorthand for `self: Self` / `self: &Self`/ `self: &mut Self`), the method takes it as`self: Box<Self>`. This syntax means the method is only valid when called on a boxed instance of the type that impl this trait (method).

The `Box` is an owning pointer, so `Box<Self>` consumes self i.e. it consumes the old state, allowing a transition into a new state. (Box invalidates the old state, so the `state` value of the `Post` can then transition into a new state).

```rust
impl Post {
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}
```

Then, we define a public `request_review` method, that takes a `&mut self`. Inside it, we'll call the internal (from trait) `request_review` method on the instance of `Post` (i.e. on the current state of `Post`). By the way, `Self::request_review` is the syntax to call the public method of the same name.

But in order to do this, we must extract the state:

To consume the old state, the public `request_review` method needs to take ownership of the `state` value. This is where the `Option` in the `state` field comes in:

> The `take` method steals the present variant out of the `Option`and puts a `None` variant back in (Rust doesn't allow leaving a struct partially initialized); it then returns the stolen variant.

> In case the variant of the Option is `Some(val)`, the `take()` method will steal it and put a `None` variant back in the Option; then it will return `Some(val)` variant. In case the Option has a `None` variant, than `None` will be returned.

In our case, `self.state.take()` will returned the stolen variant i.e. `Some(x)` where `x` has the type `Box<State>`.

To get at `x` we need to match it against `Some(s)` pattern in an `if let` expression. If matched, the `s` will hold the boxed state, on which will invoke the internal `request_review` method.

---
Then we’ll set the post’s `state` value to the result of this operation:
`self.state = Some(state.request_review())`

We need to set `state` to `None` temporarily, rather than code like `self.state = self.state.request_review()` that would set the `state` field directly, to get ownership of the state value. This ensures `Post` can’t use the old `state` value after we’ve transformed it into a new state.

The `request_review` method on `Draft` needs to return new boxed instance of `Pending` struct, which represents the state when a post is waiting for a review. The `Pending` struct also implements the `request_review` method, but doesn’t do any transformations. Rather, it returns itself, since when we request a review on a post already in the `Pending` state, it should stay in the `Pending` state.

Now we can start seeing the advantages of the state pattern: the `request_review` method on Post is the same no matter its state value. Each state is responsible for its own rules.

We’re going to leave the content method on Post as it is, returning an empty string slice. We can now have a Post in the `Pending` state as well as the Draft state, but we want the same behavior in the `Pending` state.


```rust
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(Pending {})
    }
}

struct Pending {}

impl State for Pending {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
}
```
