// bindgen-flags: --with-derive-partialeq --assume-impl-partialeq AssumePartialEq --raw-line "impl PartialEq for AssumePartialEq { fn eq(&self, other: &Self) -> bool { self.bar == other.bar } }"

/*
 * Normally AssumePartialEq will derive PartialEq but assume-impl-partialeq will
 * prevent that from happening
 */
struct AssumePartialEq {
  int bar;
};

/*
 * UsesAssumePartialEq should derive PartialEq because we assume that AssumePartialEq
 * will provide an impl for PartialEq
 */
struct UsesAssumePartialEq {
  struct AssumePartialEq foo;
  int baz;
};
