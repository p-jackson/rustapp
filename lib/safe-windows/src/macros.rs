#[macro_export]
macro_rules! define_handle_wrapper {
  ($new_type: ident, $raw_type: ty) => (
    #[derive(Debug)]
    pub enum $new_type {
      Owned($raw_type),
      Ref($raw_type)
    }
    impl $new_type {
      pub fn get_handle(&self) -> $raw_type {
        match *self {
          $new_type::Owned(h) => h,
          $new_type::Ref(h) => h
        }
      }
    }
  );
  ($new_type: ident, $raw_type: ty, $destroy_expr: expr) => (
    define_handle_wrapper!($new_type, $raw_type);
    impl std::ops::Drop for $new_type {
      fn drop(&mut self) {
        match *self {
          $new_type::Ref(_) => (),
          $new_type::Owned(h) => unsafe {
            $destroy_expr(h);
          }
        }
      }
    }
  )
}

#[macro_export]
macro_rules! handle_null {
  ($expr: expr) => (
    {
      let a = $expr;
      if a == null_mut() {
        Err(get_last_error())
      }
      else {
        Ok(a)
      }
    }
  )
}
