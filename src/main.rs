extern crate rand;
extern crate iron;
extern crate rustc_serialize;
extern crate router;

use iron::prelude::*;
use router::Router;

mod name_gen {
  use rand;
  use rand::Rng; // trait

  pub fn pick_response(name: String) -> String {
    let num = rand::thread_rng().gen_range(1, 12);

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
      _ => unreachable!()
    };

    response.to_string()
  }

  pub fn get_name(name: String) -> String {
    pick_response(name)
  }

  pub fn get_default_name() -> String {
    pick_response("Brian".to_string())
  }
}

mod handlers {
  use iron::prelude::*;
  use iron::status;
  use iron::mime::Mime;
  use rustc_serialize::json;
  use std::io::Read;
  use router::Router;
  use name_gen::{ get_name, get_default_name };

  #[derive(RustcDecodable)]
  struct JsonRequest {
    name: String
  }

  #[derive(RustcEncodable, RustcDecodable)]
  struct JsonResponse {
    name: String,
    success: bool,
    error_message: String
  }

  impl JsonResponse {
    fn success(name: String) -> Self {
      JsonResponse { name: name, success: true, error_message: "".to_string() }
    }

    fn error(msg: String) -> Self {
      JsonResponse { name: "".to_string(), success: false, error_message: msg }
    }
  }

  pub fn default(_: &mut Request) -> IronResult<Response> {
    let response = JsonResponse::success(get_default_name());
    let out = json::encode(&response).expect("Error encoding response");

    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    Ok(Response::with((content_type, status::Ok, out)))
  }

  pub fn get_handler(req: &mut Request) -> IronResult<Response> {
    let ref name = req.extensions.get::<Router>().unwrap().find("name").unwrap_or("/");
    let response = JsonResponse::success(get_name((*name).to_string()));
    let out = json::encode(&response).expect("Error encoding response");

    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    Ok(Response::with((content_type, status::Ok, out)))
  }

  pub fn post_handler(req: &mut Request) -> IronResult<Response> {
    let mut payload = String::new();
    req.body.read_to_string(&mut payload).expect("Failed to read request body");

    let out = match json::decode(&payload) {
      Err(e) => {
        let response = JsonResponse::error(format!("Error parsing JSON: {:?}", e));
        json::encode(&response).ok().expect("Error encoding response")
      },
      Ok(incoming) => {
        let converted: JsonRequest = incoming;
        let response = JsonResponse::success(get_name(converted.name));
        json::encode(&response).expect("Error encoding response")
      }
    };

    let content_type = "application/json".parse::<Mime>().expect("Failed to parse content type");
    Ok(Response::with((content_type, status::Ok, out)))
  }
}

fn main() {
  let mut router = Router::new();
  router.get("/", handlers::default, "index");
  router.get("/:name", handlers::get_handler, "name");
  router.post("/", handlers::post_handler, "post_name");

  println!("Listening on localhost:3009");
  Iron::new(router).http("localhost:3009").ok();
}

#[cfg(test)]
mod test {
  #[test]
  fn generates_name() {
    let name = super::pick_response("Brian".to_string());
    assert!(name.len() > 0);
  }
}