#[macro_export]
macro_rules! beat {
  // macro to create a beat tuple containing Decimals and bool.
  // beat!(start, stop, interval, t0act) : three f64 values and a bool, comma separated.
  // => return a tuple (start: Decimal, stop: Decimal, interval: Decimal, t0act: bool).
  ($start: expr, $stop: expr, $interval: expr, $t0act: expr) => {
    (
      Decimal::from_str(&$start.to_string()).unwrap(),
      Decimal::from_str(&$stop.to_string()).unwrap(),
      Decimal::from_str(&$interval.to_string()).unwrap(),
      $t0act,
    )
  };
}
