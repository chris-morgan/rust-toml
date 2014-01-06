use std::io::mem::MemReader;
use std::io::buffered::BufferedReader;
use std::io::File;
use toml::Parser;

mod toml;

fn main() {
  let path = Path::new(std::os::args()[1]);
  let mut file = File::open(&path);

  //let contents = file.read_to_end();
  let mut rd = BufferedReader::new(file);
  let value = Parser::parse_from_buffer(&mut rd);

  println!("{:s}", value.to_str());

  let a = value.as_ref().and_then(|a| a.lookup_key("a").and_then(|a| a.get_str()));
  if a.is_some() { println!("Found a: {:?}", a) }

  let abc_def_a = value.as_ref().and_then(|a| a.lookup_key("abc").and_then(|a| a.lookup_key("def").and_then(|a| a.lookup_key("a"))));
  if abc_def_a.is_some() { println!("Found abc.def.a: {:?}", abc_def_a) }

  let a = value.as_ref().and_then(|a| a.lookup_path(["abc", "def", "a"]));
  if a.is_some() { println!("Found a: {:?}", a) }
}
