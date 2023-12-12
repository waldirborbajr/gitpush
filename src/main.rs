mod git;
mod randmessage;
mod version;

use colorful::Colorful;
use git::gitpush;
use randmessage::rand_message;
use version::show_version;

fn main() {
  let mut param = std::env::args().skip(1);

  let message = match param.next() {
    Some(content) if !content.is_empty() => content.to_string(),
    _ => rand_message().to_string(),
  };

  // let message = if let Some(content) = param.next() {
  //   if content.is_empty() {
  //     rand_message().to_string()
  //   } else {
  //     content.to_string()
  //   }
  // } else {
  //   rand_message().to_string()
  // };

  // Display the version from Cargo
  println!("{}", show_version().green().bold());

  // add + commit + push
  gitpush(&message)
}
