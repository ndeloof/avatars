use std::net::SocketAddr;

use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, header, Method, Request, Response, StatusCode};
use tokio::net::TcpListener;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/api/avatar/spec") => Ok(Response::new(Body::from(
            r##"
            {
                "exclusions": {
                  "facial_hair_color": {
                    "key": "NONE", 
                    "part": "facial_hair"
                  }, 
                  "hair_color": {
                    "key": "NONE", 
                    "part": "top"
                  }
                }, 
                "groups": {
                  "facial_features": [
                    "eyebrows", 
                    "eyes", 
                    "mouth", 
                    "skin_color"
                  ], 
                  "hair": [
                    "top", 
                    "hair_color", 
                    "facial_hair", 
                    "facial_hair_color"
                  ]
                }, 
                "parts": {
                  "accessory": "AccessoryType", 
                  "eyebrows": "EyebrowType", 
                  "eyes": "EyeType", 
                  "facial_hair": "FacialHairType", 
                  "facial_hair_color": "HairColor", 
                  "hair_color": "HairColor", 
                  "hat_color": "ClothingColor", 
                  "mouth": "MouthType", 
                  "nose": "NoseType", 
                  "skin_color": "SkinColor", 
                  "top": "HairType"
                }, 
                "values": {
                  "AccessoryType": {
                    "EYEPATCH": "eyepatch", 
                    "KURT": "kurt", 
                    "NONE": "", 
                    "PRESCRIPTION_1": "prescription_01", 
                    "PRESCRIPTION_2": "prescription_02", 
                    "ROUND": "round", 
                    "SUNGLASSES": "sunglasses", 
                    "SUNGLASSES_2": "sunglasses_2", 
                    "WAYFARERS": "wayfarers", 
                    "WAYFARERS_2": "wayfarers_2"
                  }, 
                  "ClothingColor": {
                    "BLACK": "#262E33", 
                    "BLUE_01": "#65C9FF", 
                    "BLUE_02": "#5199E4", 
                    "BLUE_03": "#25557C", 
                    "GRAY_01": "#E6E6E6", 
                    "GRAY_02": "#929598", 
                    "HEATHER": "#3C4F5C", 
                    "PASTEL_BLUE": "#B1E2FF", 
                    "PASTEL_GREEN": "#A7FFC4", 
                    "PASTEL_ORANGE": "#FFDEB5", 
                    "PASTEL_YELLOW": "#FFFFB1", 
                    "PINK": "#FF488E", 
                    "RED": "#FF5C5C", 
                    "WHITE": "#FFFFFF"
                  }, 
                  "EyeType": {
                    "CLOSED": "closed", 
                    "CRY": "cry", 
                    "DEFAULT": "default", 
                    "EYE_ROLL": "eye_roll", 
                    "HAPPY": "happy", 
                    "HEART": "heart", 
                    "SIDE": "side", 
                    "SQUINT": "squint", 
                    "SURPRISED": "surprised", 
                    "WINK": "wink", 
                    "WINK_WACKY": "wink_wacky", 
                    "X_DIZZY": "x_dizzy"
                  }, 
                  "EyebrowType": {
                    "ANGRY": "angry", 
                    "ANGRY_NATURAL": "angry_natural", 
                    "DEFAULT": "default", 
                    "DEFAULT_NATURAL": "default_natural", 
                    "FLAT_NATURAL": "flat_natural", 
                    "FROWN_NATURAL": "frown_natural", 
                    "NONE": "", 
                    "RAISED_EXCITED": "raised_excited", 
                    "RAISED_EXCITED_NATURAL": "raised_excited_natural", 
                    "SAD_CONCERNED": "sad_concerned", 
                    "SAD_CONCERNED_NATURAL": "sad_concerned_natural", 
                    "UNIBROW_NATURAL": "unibrow_natural", 
                    "UP_DOWN": "up_down", 
                    "UP_DOWN_NATURAL": "up_down_natural"
                  }, 
                  "FacialHairType": {
                    "BEARD_LIGHT": "beard_light", 
                    "BEARD_MAGESTIC": "beard_magestic", 
                    "BEARD_MEDIUM": "beard_medium", 
                    "EINSTEIN_MOUSTACHE": "einstien_mustache", 
                    "MOUSTACHE_FANCY": "moustache_fancy", 
                    "MOUSTACHE_MAGNUM": "moustache_magnum", 
                    "NONE": "", 
                    "WICK_BEARD": "wick_beard"
                  }, 
                  "HairColor": {
                    "AUBURN": "#A55728", 
                    "BLACK": "#2C1B18", 
                    "BLONDE": "#B58143", 
                    "BLONDE_GOLDEN": "#D6B370", 
                    "BROWN": "#724133", 
                    "BROWN_DARK": "#4A312C", 
                    "PASTEL_PINK": "#F59797", 
                    "PLATINUM": "#ECDCBF", 
                    "RED": "#C93305", 
                    "SILVER_GRAY": "#E8E1E1"
                  }, 
                  "HairType": {
                    "ASTRONAUT": "astronout", 
                    "BIG_HAIR": "big_hair", 
                    "BOB": "bob", 
                    "BRAIDS": "braids", 
                    "BRIDE": "bride", 
                    "BUN": "bun", 
                    "BUZZCUT": "buzzcut", 
                    "CAESAR": "caesar", 
                    "CAESAR_SIDE_PART": "caesar_side_part", 
                    "CORNROWS": "cornrows", 
                    "CURLY": "curly", 
                    "CURLY_2": "curly_2", 
                    "CURVY": "curvy", 
                    "DREADLOCKS": "dreadlocks", 
                    "DREADS": "dreads", 
                    "EINSTEIN_HAIR": "einstein_hair", 
                    "ELVIS": "elvis", 
                    "EVIL_SPIKE": "evil_spike", 
                    "FRIDA": "frida", 
                    "FRIZZLE": "frizzle", 
                    "FRO": "fro", 
                    "FRO_BAND": "fro_band", 
                    "HALF_SHAVED": "half_shaved", 
                    "HAT": "hat", 
                    "LONG_HAIR_CURLY": "long_hair_curly", 
                    "LONG_NOT_TOO_LONG": "long_not_too_long", 
                    "LOOSE_HAIR": "loose_hair", 
                    "MIA_WALLACE": "mia_wallace", 
                    "MOHAWK": "mohawk", 
                    "MOWGLI": "mowgli", 
                    "NONE": "no_hair", 
                    "PIXIE": "pixie", 
                    "POMPADOUR": "pompadour", 
                    "QUIFF": "quiff", 
                    "SHAGGY": "shaggy", 
                    "SHAGGY_MULLET": "shaggy_mullet", 
                    "SHAVED_SIDES": "shaved_sides", 
                    "SHORT_CURLY": "short_curly", 
                    "SHORT_DREADS_1": "short_dreads_1", 
                    "SHORT_DREADS_2": "short_dreads_2", 
                    "SHORT_FLAT": "short_flat", 
                    "SHORT_ROUND": "short_round", 
                    "SHORT_WAVED": "short_waved", 
                    "SIDES": "sides", 
                    "STRAIGHT_1": "straight_1", 
                    "STRAIGHT_2": "straight_2", 
                    "STRAIGHT_STRAND": "straight_strand", 
                    "TWIST_LONG_HAIR": "twist_long_hair", 
                    "TWIST_LONG_HAIR_2": "twist_long_hair_2", 
                    "WICK": "wick", 
                    "WILD": "wild"
                  }, 
                  "MouthType": {
                    "BIG_SMILE": "big_smile", 
                    "CONCERNED": "concerned", 
                    "DEFAULT": "default", 
                    "DISBELIEF": "disbelief", 
                    "EATING": "eating", 
                    "GRIMACE": "grimace", 
                    "SAD": "sad", 
                    "SCREAM_OPEN": "scream_open", 
                    "SERIOUS": "serious", 
                    "SMILE": "smile", 
                    "TONGUE": "tongue", 
                    "TWINKLE": "twinkle", 
                    "VOMIT": "vomit"
                  }, 
                  "NoseType": {
                    "DEFAULT": "default", 
                    "SMALL": "small", 
                    "WIDE": "wide"
                  }, 
                  "SkinColor": {
                    "BLACK": "#614335", 
                    "BROWN": "#D08B5B", 
                    "DARK_BROWN": "#AE5D29", 
                    "LIGHT": "#EDB98A", 
                    "PALE": "#FFDBB4", 
                    "TANNED": "#FD9841", 
                    "YELLOW": "#F8D25C"
                  }
                }
              }            
            "##))),
          
        (&Method::GET, "/api/avatar") => Ok(Response::builder()
          .status(StatusCode::OK)
          .header(header::CONTENT_TYPE, "image/svg+xml")
          .body(Body::from(r##"<?xml version="1.0" encoding="utf-8"?><!-- Uploaded to: SVG Repo, www.svgrepo.com, Generator: SVG Repo Mixer Tools -->
<svg width="800px" height="800px" viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg"><title>file_type_wasm</title><path d="M19.153,2.35V2.5a3.2,3.2,0,1,1-6.4,0h0V2.35H2V30.269H29.919V2.35Z" style="fill:#654ff0"/><path d="M8.485,17.4h1.85L11.6,24.123h.023L13.14,17.4h1.731l1.371,6.81h.027l1.44-6.81h1.815l-2.358,9.885H15.329l-1.36-6.728h-.036l-1.456,6.728h-1.87Zm13.124,0h2.917l2.9,9.885H25.515l-.63-2.2H21.562l-.486,2.2H19.217Zm1.11,2.437-.807,3.627h2.512L23.5,19.832Z" style="fill:#fff"/></svg>
"##
        )).unwrap()),

        // Return the 404 Not Found for other routes.
        _ => {
          let mut not_found = Response::default();
          *not_found.status_mut() = StatusCode::NOT_FOUND;
          Ok(not_found)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new().serve_connection(stream, service_fn(echo)).await {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
