extern crate rand;
extern crate iron;
extern crate rustc_serialize;
extern crate router;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use rand::Rng;
use rustc_serialize::json;
use router::Router;
use std::io::Read;

#[derive(RustcEncodable, RustcDecodable)]
struct JsonResponse {
  name: String
}

fn pick_response(name: String) -> String {
  let num = rand::thread_rng().gen_range(1, 12);

  // let response = match num {
  //   1 => "Hello World!",
  //   2 => "Did you see that ludicrous display last night?",
  //   3 => "Nice weather for ducks",
  //   _ => ""
  // };

  let prefixes = vec!["Asylum", "Bell", "Black", "Capra", "Ceaseless", "Centipede", "Chaos", "Crossbreed", "Dark Sun", "Slayer", "Executioner", "Gaping", "Gravelord", "Iron", "Cinder", "Father", "Abyss", "Moonlight", "Sanctuary", "Stray", "Taurus", "Armoured", "Golden", "Crystal", "Giant", "Undead", "Giant Undead", "Hellkite", "Parasitic", "Prowling", "Prince", "Grey", "Maneater", "Iron", "Paladin", "Xanthous", "Marvellous", "Big Hat", "Black Iron", "Crestfallen", "Darkstalker", "Gravelord", "Hawkeye", "Kingseeker", "Lord\"s Blade", "Stone", "Silent", "Belfry", "Captain", "Emerald", "Grave Warden", "Lonesome", "Manscorpion", "Hag", "Mild Mannered", "Royal", "Sorcerer", "Sparkling", "Steady Hand", "Old", "Ruin", "Old Iron", "Covetous", "Baleful", "Prowling", "Ancient", "Burnt", "Slumbering", "Ivory", "Fume", "Sir", "Nameless", "Pilgrim", "Jester", "Ashen", "Abbess", "Rapacious", "Drifter", "Woodland Child", "Peculiar", "Holy", "Yellowfinger", "Longfinger", "Knight-Slayer", "Curse-Rotted", "Deacon", "High Lord", "Old Demon", "Pontiff", "Boreal", "Unbreakable", "Ringfinger"];
  let types = vec!["Demon", "Gargoyle", "Dragon", "Witch", "Golem", "Knight", "Wolf", "Butcher", "Tusk", "Golem", "Rat", "Hydra", "Wall Hugger", "Prince", "Slayer", "King", "Blacksmith", "Princess", "Merchant", "Scholar", "Oracle", "Guard", "Captain", "Chancellor", "Herald", "Housekeeper", "Laddersmith", "Manscorpion", "Warrior", "Trader", "Lord", "Sentinel", "Queen", "Ogre", "Denizen", "Seeker", "Watcher", "Devourer", "Outrider Knight", "High Priestess", "Mother"];
  let suffixes = vec!["of Chaos", "of the Abyss", "of Cinder", "of Thorns", "of the Darkroot Wood", "of Astora", "of Zena", "of Oolacile", "of Vinheim", "of Sunlight", "of Carim", "of the Great Swamp", "of Thorolund", "of Izalith", "of the East", "of Catarina", "of the First Sin", "of Jugo", "of Mirrah", "of Lanafir", "of Olaphis", "of Song", "of Londor", "of the Spurned", "of the Sunless Realms", "of Carim", "of the Boreal Valley", "of the Deep", "of Lothric Castle", "of Courland", "of Rebirth"];
  let nicknames = vec!["the Scaleless", "the Great", "the Rock", "the Crow", "the Cartographer", "the Wanderer", "the Pardoner", "the Outcast", "the Armourer", "the Crestfallen", "the Lost", "the Ruined", "the Baleful", "the King\"s Pet", "the Squalid", "the Explorer", "the Butcher", "the Deserter", "the Hushed", "the Giant", "the Consumed"];

  let prefix = prefixes[rand::thread_rng().gen_range(0, prefixes.len())];
  let name_type = types[rand::thread_rng().gen_range(0, types.len())];
  let suffix = suffixes[rand::thread_rng().gen_range(0, suffixes.len())];
  let nickname = nicknames[rand::thread_rng().gen_range(0, nicknames.len())];

  let response = match num {
    1 => format!("{} {}", prefix, name_type),
    2 => format!("{} {}", prefix, name),
    3 => format!("{} {}, {} {}", name, nickname, prefix, name_type),
    4 => format!("{}, {} {}", name, name_type, suffix),
    5 => format!("{} {}", name, suffix),
    6 => format!("{} {} {}", name, nickname, suffix),
    7 => format!("{} {} {}", prefix, name_type, name),
    8 => format!("{} {} {}", prefix, name, nickname),
    9 => format!("{} {}, {} {}", prefix, name, name_type, suffix),
    10 => format!("{}, {} {}", name, nickname, name_type),
    11 => format!("{} {}", name_type, name),
    _ => format!("{}", name)
  };

  response.to_string()
}

fn get_name(name: String) -> String {
  pick_response(name)
}

fn get_default_name() -> String {
  pick_response("Brian".to_string())
}

fn handler(req: &mut Request) -> IronResult<Response> {
  let response = JsonResponse { name: get_default_name() };
  let out = json::encode(&response).unwrap();

  let content_type = "application/json".parse::<Mime>().unwrap();
  Ok(Response::with((content_type, status::Ok, out)))
}

fn get_handler(req: &mut Request) -> IronResult<Response> {
  let ref name = req.extensions.get::<Router>().unwrap().find("name").unwrap_or("/");

  let response = JsonResponse { name: get_name((*name).to_string()) };
  let out = json::encode(&response).unwrap();

  let content_type = "application/json".parse::<Mime>().unwrap();
  Ok(Response::with((content_type, status::Ok, out)))
}

fn post_handler(req: &mut Request) -> IronResult<Response> {
  let mut payload = String::new();
  req.body.read_to_string(&mut payload).unwrap();
  println!("{:?}", payload);

  let incoming: JsonResponse = json::decode(&payload).unwrap();

  let response = JsonResponse { name: get_name(incoming.name) };
  let out = json::encode(&response).unwrap();

  let content_type = "application/json".parse::<Mime>().unwrap();
  Ok(Response::with((content_type, status::Ok, out)))
}

fn main() {
  let mut router = Router::new();
  router.get("/", handler, "index");
  router.get("/:name", get_handler, "name");
  router.post("/", post_handler, "post_name");

  Iron::new(router).http("localhost:3009").unwrap();
}
