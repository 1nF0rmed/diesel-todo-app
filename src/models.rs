#[derive(Queryable)]
pub struct Task {
  pub id: String,
  pub title: String,
  pub descp: String,
  pub completed: bool,
}