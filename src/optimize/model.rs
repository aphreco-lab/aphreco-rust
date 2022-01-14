use super::result::SimResult;

pub trait OptModelTraitFix<
  const LEN_Y: usize,
  const LEN_P: usize,
  const LEN_B: usize,
  const LEN_X: usize,
>
{
  fn getx(&self) -> &[f64; LEN_P];
}
