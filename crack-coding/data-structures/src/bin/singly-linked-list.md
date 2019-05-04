## Info and FAQs

- Why Did we use three different structs with similar looking data inside

* I was first confused, that why do we need `List` and a `Node` and an `Iter`
  structs separately, but it got clear step by step. We need `List` which can
  act like a public module, containing our header node, that's it.
  `Node` represents a structure of any Node present inside Linked List (singly, doubly)
  doesn't matter. `Iter` and also `IterMut` if needed, are structures that are not linked to
  `Node` or `List` Directly, but provide a `non-mutable` or `mutable` reference, that user can work
  on inside a loop.
