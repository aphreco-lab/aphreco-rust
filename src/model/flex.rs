pub trait FlexModelSimTrait<const LEN_Y: usize, const LEN_P: usize> {}

pub trait FlexModelOptTrait<const LEN_Y: usize, const LEN_P: usize, const LEN_X: usize>:
  FlexModelSimTrait<LEN_Y, LEN_P>
{
}
