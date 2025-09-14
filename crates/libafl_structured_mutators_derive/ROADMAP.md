# libafl_structured_mutators_derive RoadMap

- [ ] Mutating simple flat struct with trivial fields.
- [ ] Recursive mutator
- [ ] choosing fields based on `complexity` of fields that the mutator determines.
- [ ] Splice mutator:
    - [ ] For example, there is a Vec<D> somewhere inside the input I. we want to
          somehow get access to all other `D` types in the rest of the corpus in order
          to implement splicing.
    - [ ] Also when there is `BytesBuffer` we want to fetch another `BytesBuffer` from the corpus. 
    - [ ] Maybe just impl `HasCorpus<D>` for every `D` that needs splice mutator in `I`?