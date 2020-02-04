use exitfailure::ExitFailure;
use failure::ResultExt;
use std::fs;

pub fn run(path: std::path::PathBuf, write: bool) -> Result<(), ExitFailure> {
  let path_name = path.clone().into_os_string();
  let is_simulation = !write;
  if is_simulation {
    println!("Simulation mode only!");
  }

  let dir =
    fs::read_dir(path).with_context(|_| format!("Could not find directory {:?}", path_name))?;

  println!("Directory {:?}", dir);

  Ok(())

  // let pb = indicatif::ProgressBar::new(100);
  // for _ in 0..100 {
  //     pb.inc(1);
  // }
  // pb.finish_with_message("done");
}

fn answer() -> i32 {
  42
}

#[test]
fn check_answer_validity() {
  assert_eq!(answer(), 42);
}
