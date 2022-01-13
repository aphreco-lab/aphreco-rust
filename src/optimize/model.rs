pub trait OptTraitFlex<const LEN_Y: usize, const LEN_P: usize, const LEN_X: usize>:
  FlexModelSimTrait<LEN_Y, LEN_P>
{
}

pub trait OptTraitOde<const LEN_Y: usize, const LEN_P: usize, const LEN_X: usize> {
  fn getx(&self);
}

pub trait OptTraitFix<
  const LEN_Y: usize,
  const LEN_P: usize,
  const LEN_B: usize,
  const LEN_X: usize,
>: FixModelSimTrait<LEN_Y, LEN_P, LEN_B> + Clone + Send + 'static
{
  // getx(&self) -> (x_index, x_bounds) {}
  // x_index indicates which parameter will be optimized and it is used when optimizer renews parameter values.
  // x_bounds are for generating initial value cluster of x in methods not using initial values.
  // In methods using initial values will use the values in p as initial values instead of the bounds.
  fn getx(&self) -> (Vec<usize>, Option<Vec<(f64, f64)>>);

  // setx(&self, index: usize, value: f64) {}
  // set a value to p[index] in a model.
  fn setp(&mut self, _index: usize, _value: f64) {
    unimplemented!(
      "\nplease implement setp function in OptModelTrait:\n
fn setp(&mut self, index: usize, value: f64) {{
self.p[index] = value;
}}\n
"
    );
  }
}
