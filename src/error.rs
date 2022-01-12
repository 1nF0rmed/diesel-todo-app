use failure::Fail;

#[derive(Debug, Fail)]
pub enum TodoAppError {
  #[fail(display = "Diesel error: {}", _0)]
  DieselError(diesel::result::Error)
}

pub type Result<T> = std::result::Result<T, TodoAppError>;

impl From<diesel::result::Error> for TodoAppError {
  fn from(e: diesel::result::Error) -> Self {
      TodoAppError::DieselError(e)
  }
}
