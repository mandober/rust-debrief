The trait is known as the implemented trait.

The implementing type implements the implemented trait.

A trait implementation must define all non-default associated items declared by the implemented trait, may redefine default associated items defined by the implemented trait, and cannot define any other items.

The path to the associated items is < followed by a path to the implementing type followed by as followed by a path to the trait followed by > as a path component followed by the associated item's path component.
