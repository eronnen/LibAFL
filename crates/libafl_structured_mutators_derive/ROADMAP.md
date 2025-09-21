# libafl_structured_mutators_derive RoadMap

- [x] Mutating simple flat struct with integer fields.
- [x] Recursive mutator
- [x] choosing fields based on `weight` of fields that the mutator determines.
- [x] Implement mutators for more basic types:
      - [x] `bool`
      - [x] `char`
      - [x] `Option<T>`
- [x] Implement `Vec<T>` mutator
- [ ] Implement default `StructuredMutator` for Enum types.
- [ ] Crossover mutator:
    - [ ] For example, there is a Vec<D> somewhere inside the input I. we want to
          somehow get access to all other `D` types in the rest of the corpus in order
          to implement splicing.
    - [ ] Also when there is `BytesBuffer` we want to fetch another `BytesBuffer` from the corpus. 
    - [ ] Maybe just impl `HasCorpus<D>` for every `D` that needs splice mutator in `I`?
- [ ] Implement bridge mutator for `BytesInput` that will call LibAFL mutator
- [ ] Incorporate crossover with default structured mutators instead of ::default
      - [ ] `Vec<T>`
            - [ ] crossover insert one
            - [ ] crossover insert multiple
            - [ ] crossover replace
      - [ ] `Option<T>`
- [ ] Have probabilities distributions for grouped mutators like `Vec` and `char`.
- [ ] Implement generic `CompositionMutator` that chooses with distribution over a list of mutators.
- [ ] Allow dynamic injection by the user to replace default mutators
- [ ] Unnamed tuple mutators.
- [ ] Add parameters to the `StructureMutate` proc-macro.