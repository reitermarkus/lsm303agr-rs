pub trait RegRead<D = u8> {
    type Output;

    const ADDR: u8;

    fn from_data(data: D) -> Self::Output;
}

pub trait RegWrite<D = u8>: RegRead<D> {
    fn data(&self) -> D;
}

macro_rules! register {
  (@impl_reg_read $ty:ident, $addr:literal, $output:ident) => {
    impl $crate::reg::RegRead for $ty {
      type Output = $output;

      const ADDR: u8 = $addr;

      fn from_data(data: u8) -> Self::Output {
        Self::Output::from_bits_truncate(data)
      }
    }
  };
  (@impl_reg_write $ty:ident, $addr:literal, $output:ident) => {
    register!(@impl_reg_read $ty, $addr, Self);

    impl $crate::reg::RegWrite for $ty {
      fn data(&self) -> u8 {
        self.bits()
      }
    }
  };
  (
    #[doc = $name:expr]
    $(#[$meta:meta])*
    $vis:vis type $ty:ident: $addr:literal = $ty2:ident;
  ) => {
    #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
    $(#[$meta])*
    $vis enum $ty {}

    register!(@impl_reg_read $ty, $addr, $ty2);
  };
  (
    #[doc = $name:expr]
    $(#[$meta:meta])*
    $vis:vis struct $ty:ident: $addr:literal {
      $(const $bit_name:ident = $bit_val:expr;)*
  }
  ) => {
    ::bitflags::bitflags! {
      #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
      $(#[$meta])*
      $vis struct $ty: u8 {
        $(const $bit_name = $bit_val;)*
      }
    }

    register!(@impl_reg_write $ty, $addr, Self);
  };
}

pub(crate) use register;
