# Design Patterns

## Component Architecture

aka Plug-In Architecture
aka Modular Application Architecture


Drop-in plugins
plugins placed in a particular directory, with the required script as the entry point.

Component architecture
tasks are delegated to modules that advertise extension points. then impl other components which would fit into these points and change the flow.

Hooks
hooks being called at every level; override hooks in files/functions named according to a convention and alter the behaviour.


Generally:
- keep non-extensible, non-user-modifiable core as small as possible.
- Delegate everything to a higher layer so extensibility increases.
- Less stuff to correct in the core then in case of bad choices.
- not too many decisions about the direction of project at the outset.
- Implement the smallest needed subset, then start writing plugins.
- embed a fill scripting language (guile)


## Example projects

py.test (python)
[http://pytest.org/latest/index.html]

asdf (bash)
Extendable version manager
[https://github.com/asdf-vm/asdf]

Trac Component Architecture (python)
[https://trac.edgewall.org/wiki/TracDev/ComponentArchitecture]

Eclipse (java)
[http://help.eclipse.org/kepler/index.jsp?topic=/org.eclipse.platform.doc.isv/reference/api/org/eclipse/core/runtime/Plugin.html]




## Links

QA
- https://stackoverflow.com/questions/44708483/idiomatic-rust-plugin-system
- https://stackoverflow.com/questions/2768104/how-to-create-a-flexible-plug-in-architecture


Modular Application Architecture - Considerations on Design Patterns
[https://www.codementor.io/goetas/modular-application-architecture-considerations-on-design-patterns-flmx1m731]

The Universal Design Pattern: Properties Pattern aka Prototype Pattern
[http://steve-yegge.blogspot.com/2008/10/universal-design-pattern.html]

